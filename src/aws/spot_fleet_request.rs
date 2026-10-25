#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SpotFleetRequestData_ {
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
    allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excess_capacity_termination_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_type: std::option::Option<terrars::PrimField<std::string::String>>,
    iam_fleet_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_interruption_behaviour: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_pools_to_use_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_total_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_target_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_unhealthy_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    target_capacity: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_capacity_unit_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_instances_on_delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_instances_with_expiration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_until: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_fulfillment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_specification: std::option::Option<std::vec::Vec<SpotFleetRequest_LaunchSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_config: std::option::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_maintenance_strategies: std::option::Option<std::vec::Vec<SpotFleetRequest_SpotMaintenanceStrategies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SpotFleetRequest_Timeouts>,
    dynamic: SpotFleetRequestDynamic,
}

struct SpotFleetRequest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpotFleetRequestData_>,
}

#[derive(Clone)]
pub struct SpotFleetRequest(std::rc::Rc<SpotFleetRequest_>);

impl SpotFleetRequest {
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

    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `context`.\n"]
    pub fn set_context(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excess_capacity_termination_policy`.\n"]
    pub fn set_excess_capacity_termination_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().excess_capacity_termination_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fleet_type`.\n"]
    pub fn set_fleet_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fleet_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_interruption_behaviour`.\n"]
    pub fn set_instance_interruption_behaviour(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_interruption_behaviour = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_pools_to_use_count`.\n"]
    pub fn set_instance_pools_to_use_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().instance_pools_to_use_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancers`.\n"]
    pub fn set_load_balancers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().load_balancers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_allocation_strategy`.\n"]
    pub fn set_on_demand_allocation_strategy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().on_demand_allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_total_price`.\n"]
    pub fn set_on_demand_max_total_price(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().on_demand_max_total_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_target_capacity`.\n"]
    pub fn set_on_demand_target_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().on_demand_target_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replace_unhealthy_instances`.\n"]
    pub fn set_replace_unhealthy_instances(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replace_unhealthy_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().spot_price = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `target_capacity_unit_type`.\n"]
    pub fn set_target_capacity_unit_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().target_capacity_unit_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arns`.\n"]
    pub fn set_target_group_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().target_group_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terminate_instances_on_delete`.\n"]
    pub fn set_terminate_instances_on_delete(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().terminate_instances_on_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terminate_instances_with_expiration`.\n"]
    pub fn set_terminate_instances_with_expiration(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instances_with_expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_from`.\n"]
    pub fn set_valid_from(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().valid_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_until`.\n"]
    pub fn set_valid_until(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().valid_until = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_fulfillment`.\n"]
    pub fn set_wait_for_fulfillment(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_fulfillment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_specification`.\n"]
    pub fn set_launch_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template_config`.\n"]
    pub fn set_launch_template_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spot_maintenance_strategies`.\n"]
    pub fn set_spot_maintenance_strategies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotFleetRequest_SpotMaintenanceStrategies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spot_maintenance_strategies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spot_maintenance_strategies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SpotFleetRequest_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `excess_capacity_termination_policy` after provisioning.\n"]
    pub fn excess_capacity_termination_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.excess_capacity_termination_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_fleet_role` after provisioning.\n"]
    pub fn iam_fleet_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_fleet_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behaviour` after provisioning.\n"]
    pub fn instance_interruption_behaviour(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behaviour", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_pools_to_use_count` after provisioning.\n"]
    pub fn instance_pools_to_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_pools_to_use_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_balancers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_allocation_strategy` after provisioning.\n"]
    pub fn on_demand_allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_allocation_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_max_total_price` after provisioning.\n"]
    pub fn on_demand_max_total_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_max_total_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_target_capacity` after provisioning.\n"]
    pub fn on_demand_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_target_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replace_unhealthy_instances` after provisioning.\n"]
    pub fn replace_unhealthy_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace_unhealthy_instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_request_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_capacity_unit_type` after provisioning.\n"]
    pub fn target_capacity_unit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_capacity_unit_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instances_on_delete` after provisioning.\n"]
    pub fn terminate_instances_on_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_on_delete", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_maintenance_strategies` after provisioning.\n"]
    pub fn spot_maintenance_strategies(&self) -> terrars::ListRef<SpotFleetRequest_SpotMaintenanceStrategiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_maintenance_strategies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotFleetRequest_TimeoutsRef {
        SpotFleetRequest_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SpotFleetRequest {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpotFleetRequest { }

impl terrars::ToListMappable for SpotFleetRequest {
    type O = terrars::ListRef<SpotFleetRequestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpotFleetRequest_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_spot_fleet_request".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpotFleetRequest {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub iam_fleet_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_capacity: terrars::PrimField<f64>,
}

impl BuildSpotFleetRequest {
    pub fn build(self, stack: &mut terrars::Stack) -> SpotFleetRequest {
        let out = SpotFleetRequest(std::rc::Rc::new(SpotFleetRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpotFleetRequestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allocation_strategy: core::default::Default::default(),
                context: core::default::Default::default(),
                excess_capacity_termination_policy: core::default::Default::default(),
                fleet_type: core::default::Default::default(),
                iam_fleet_role: self.iam_fleet_role,
                id: core::default::Default::default(),
                instance_interruption_behaviour: core::default::Default::default(),
                instance_pools_to_use_count: core::default::Default::default(),
                load_balancers: core::default::Default::default(),
                on_demand_allocation_strategy: core::default::Default::default(),
                on_demand_max_total_price: core::default::Default::default(),
                on_demand_target_capacity: core::default::Default::default(),
                region: core::default::Default::default(),
                replace_unhealthy_instances: core::default::Default::default(),
                spot_price: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_capacity: self.target_capacity,
                target_capacity_unit_type: core::default::Default::default(),
                target_group_arns: core::default::Default::default(),
                terminate_instances_on_delete: core::default::Default::default(),
                terminate_instances_with_expiration: core::default::Default::default(),
                valid_from: core::default::Default::default(),
                valid_until: core::default::Default::default(),
                wait_for_fulfillment: core::default::Default::default(),
                launch_specification: core::default::Default::default(),
                launch_template_config: core::default::Default::default(),
                spot_maintenance_strategies: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpotFleetRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpotFleetRequestRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `excess_capacity_termination_policy` after provisioning.\n"]
    pub fn excess_capacity_termination_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.excess_capacity_termination_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_fleet_role` after provisioning.\n"]
    pub fn iam_fleet_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_fleet_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behaviour` after provisioning.\n"]
    pub fn instance_interruption_behaviour(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behaviour", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_pools_to_use_count` after provisioning.\n"]
    pub fn instance_pools_to_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_pools_to_use_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_balancers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_allocation_strategy` after provisioning.\n"]
    pub fn on_demand_allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_allocation_strategy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `on_demand_max_total_price` after provisioning.\n"]
    pub fn on_demand_max_total_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_max_total_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_target_capacity` after provisioning.\n"]
    pub fn on_demand_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_target_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replace_unhealthy_instances` after provisioning.\n"]
    pub fn replace_unhealthy_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace_unhealthy_instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_request_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_capacity_unit_type` after provisioning.\n"]
    pub fn target_capacity_unit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_capacity_unit_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instances_on_delete` after provisioning.\n"]
    pub fn terminate_instances_on_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_on_delete", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_maintenance_strategies` after provisioning.\n"]
    pub fn spot_maintenance_strategies(&self) -> terrars::ListRef<SpotFleetRequest_SpotMaintenanceStrategiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_maintenance_strategies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotFleetRequest_TimeoutsRef {
        SpotFleetRequest_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SpotFleetRequestDynamic {
    launch_specification: std::option::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchSpecification>>,
    launch_template_config: std::option::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig>>,
    spot_maintenance_strategies: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_SpotMaintenanceStrategies>>,
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchSpecification_EbsBlockDevice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    device_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotFleetRequest_LaunchSpecification_EbsBlockDevice {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchSpecification_EbsBlockDevice {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification_EbsBlockDevice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchSpecification_EbsBlockDevice {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildSpotFleetRequest_LaunchSpecification_EbsBlockDevice {
    pub fn build(self) -> SpotFleetRequest_LaunchSpecification_EbsBlockDevice {
        SpotFleetRequest_LaunchSpecification_EbsBlockDevice {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchSpecification_EbsBlockDeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchSpecification_EbsBlockDeviceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchSpecification_EbsBlockDeviceRef {
        SpotFleetRequest_LaunchSpecification_EbsBlockDeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchSpecification_EbsBlockDeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice {
    device_name: terrars::PrimField<std::string::String>,
    virtual_name: terrars::PrimField<std::string::String>,
}

impl SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice { }

impl terrars::ToListMappable for SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchSpecification_EphemeralBlockDevice {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_name: terrars::PrimField<std::string::String>,
}

impl BuildSpotFleetRequest_LaunchSpecification_EphemeralBlockDevice {
    pub fn build(self) -> SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice {
        SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchSpecification_EphemeralBlockDeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchSpecification_EphemeralBlockDeviceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchSpecification_EphemeralBlockDeviceRef {
        SpotFleetRequest_LaunchSpecification_EphemeralBlockDeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchSpecification_EphemeralBlockDeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchSpecification_RootBlockDevice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotFleetRequest_LaunchSpecification_RootBlockDevice {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchSpecification_RootBlockDevice {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification_RootBlockDevice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchSpecification_RootBlockDevice {}

impl BuildSpotFleetRequest_LaunchSpecification_RootBlockDevice {
    pub fn build(self) -> SpotFleetRequest_LaunchSpecification_RootBlockDevice {
        SpotFleetRequest_LaunchSpecification_RootBlockDevice {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchSpecification_RootBlockDeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchSpecification_RootBlockDeviceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchSpecification_RootBlockDeviceRef {
        SpotFleetRequest_LaunchSpecification_RootBlockDeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchSpecification_RootBlockDeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotFleetRequest_LaunchSpecificationDynamic {
    ebs_block_device: std::option::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchSpecification_EbsBlockDevice>>,
    ephemeral_block_device: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice>>,
    root_block_device: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchSpecification_RootBlockDevice>>,
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchSpecification {
    ami: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associate_public_ip_address: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_optimized: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_instance_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_instance_profile_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_tenancy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_block_device: std::option::Option<std::vec::Vec<SpotFleetRequest_LaunchSpecification_EbsBlockDevice>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_block_device: std::option::Option<std::vec::Vec<SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_block_device: std::option::Option<std::vec::Vec<SpotFleetRequest_LaunchSpecification_RootBlockDevice>>,
    dynamic: SpotFleetRequest_LaunchSpecificationDynamic,
}

impl SpotFleetRequest_LaunchSpecification {
    #[doc = "Set the field `associate_public_ip_address`.\n"]
    pub fn set_associate_public_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.associate_public_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized`.\n"]
    pub fn set_ebs_optimized(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ebs_optimized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_instance_profile`.\n"]
    pub fn set_iam_instance_profile(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.iam_instance_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_instance_profile_arn`.\n"]
    pub fn set_iam_instance_profile_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.iam_instance_profile_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.placement_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_tenancy`.\n"]
    pub fn set_placement_tenancy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.placement_tenancy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spot_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vpc_security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weighted_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification_EbsBlockDevice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_block_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_block_device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification_EphemeralBlockDevice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ephemeral_block_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_block_device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification_RootBlockDevice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.root_block_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.root_block_device = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchSpecification {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchSpecification {
    #[doc = ""]
    pub ami: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildSpotFleetRequest_LaunchSpecification {
    pub fn build(self) -> SpotFleetRequest_LaunchSpecification {
        SpotFleetRequest_LaunchSpecification {
            ami: self.ami,
            associate_public_ip_address: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            ebs_optimized: core::default::Default::default(),
            iam_instance_profile: core::default::Default::default(),
            iam_instance_profile_arn: core::default::Default::default(),
            instance_type: self.instance_type,
            key_name: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            placement_group: core::default::Default::default(),
            placement_tenancy: core::default::Default::default(),
            spot_price: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            user_data: core::default::Default::default(),
            vpc_security_group_ids: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            ebs_block_device: core::default::Default::default(),
            ephemeral_block_device: core::default::Default::default(),
            root_block_device: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotFleetRequest_LaunchSpecificationRef {
        SpotFleetRequest_LaunchSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami", self.base))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile_arn` after provisioning.\n"]
    pub fn iam_instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_tenancy` after provisioning.\n"]
    pub fn placement_tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_tenancy", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {
        SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
        SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {
    type O =
        terrars
        ::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCountRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {
    pub fn build(
        self,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMibRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {
    type O =
        terrars
        ::BlockAssignable<
            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {
    pub fn build(
        self,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {
    type O =
        terrars
        ::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpuRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {
    type O =
        terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMibRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMibRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {
    type O =
        terrars
        ::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbpsRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {
    type O =
        terrars
        ::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCountRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {
    type O =
        terrars
        ::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGbRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {
    type O =
        terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCountRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsDynamic {
    accelerator_count: std
    ::option
    ::Option<
        terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount>,
    >,
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib>,
    >,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps>,
    >,
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu>,
    >,
    memory_mib: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib>>,
    network_bandwidth_gbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps>,
    >,
    network_interface_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount>,
    >,
    total_local_storage_gb: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb>,
    >,
    vcpu_count: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount>>,
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bare_metal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burstable_performance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_generations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps: std
    ::option
    ::Option<
        std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<
        std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount>>,
    dynamic: SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsDynamic,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bare_metal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.burstable_performance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cpu_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_generations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.local_storage_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_hibernate_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMib,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbps,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpu,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMib,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbps,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interface_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGb,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vcpu_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsRef {
        SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_names", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_types", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.burstable_performance", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cpu_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_generations", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_storage_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_hibernate_support", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars
    ::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_AcceleratorTotalMemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars
    ::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_BaselineEbsBandwidthMbpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryGibPerVcpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_MemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> terrars
    ::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkBandwidthGbpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_bandwidth_gbps", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars
    ::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_NetworkInterfaceCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars
    ::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_TotalLocalStorageGbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements_VcpuCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotFleetRequest_LaunchTemplateConfig_OverridesDynamic {
    instance_requirements: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements>>,
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig_Overrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements>>,
    dynamic: SpotFleetRequest_LaunchTemplateConfig_OverridesDynamic,
}

impl SpotFleetRequest_LaunchTemplateConfig_Overrides {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spot_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weighted_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirements>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_requirements = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchTemplateConfig_Overrides {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig_Overrides {}

impl BuildSpotFleetRequest_LaunchTemplateConfig_Overrides {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig_Overrides {
        SpotFleetRequest_LaunchTemplateConfig_Overrides {
            availability_zone: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            priority: core::default::Default::default(),
            spot_price: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfig_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfig_OverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_LaunchTemplateConfig_OverridesRef {
        SpotFleetRequest_LaunchTemplateConfig_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfig_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_LaunchTemplateConfig_Overrides_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotFleetRequest_LaunchTemplateConfigDynamic {
    launch_template_specification: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification>>,
    overrides: std::option::Option<terrars::DynamicBlock<SpotFleetRequest_LaunchTemplateConfig_Overrides>>,
}

#[derive(Serialize)]
pub struct SpotFleetRequest_LaunchTemplateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<std::vec::Vec<SpotFleetRequest_LaunchTemplateConfig_Overrides>>,
    dynamic: SpotFleetRequest_LaunchTemplateConfigDynamic,
}

impl SpotFleetRequest_LaunchTemplateConfig {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig_Overrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_LaunchTemplateConfig {
    type O = terrars::BlockAssignable<SpotFleetRequest_LaunchTemplateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_LaunchTemplateConfig {}

impl BuildSpotFleetRequest_LaunchTemplateConfig {
    pub fn build(self) -> SpotFleetRequest_LaunchTemplateConfig {
        SpotFleetRequest_LaunchTemplateConfig {
            launch_template_specification: core::default::Default::default(),
            overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_LaunchTemplateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_LaunchTemplateConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotFleetRequest_LaunchTemplateConfigRef {
        SpotFleetRequest_LaunchTemplateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_LaunchTemplateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_LaunchTemplateConfig_LaunchTemplateSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {
    #[doc = "Set the field `replacement_strategy`.\n"]
    pub fn set_replacement_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replacement_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {
    type O = terrars::BlockAssignable<SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {}

impl BuildSpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {
    pub fn build(self) -> SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {
        SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance {
            replacement_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalanceRef {
        SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replacement_strategy` after provisioning.\n"]
    pub fn replacement_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement_strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpotFleetRequest_SpotMaintenanceStrategiesDynamic {
    capacity_rebalance: std
    ::option
    ::Option<terrars::DynamicBlock<SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance>>,
}

#[derive(Serialize)]
pub struct SpotFleetRequest_SpotMaintenanceStrategies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_rebalance: std::option::Option<std::vec::Vec<SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance>>,
    dynamic: SpotFleetRequest_SpotMaintenanceStrategiesDynamic,
}

impl SpotFleetRequest_SpotMaintenanceStrategies {
    #[doc = "Set the field `capacity_rebalance`.\n"]
    pub fn set_capacity_rebalance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_rebalance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_rebalance = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpotFleetRequest_SpotMaintenanceStrategies {
    type O = terrars::BlockAssignable<SpotFleetRequest_SpotMaintenanceStrategies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_SpotMaintenanceStrategies {}

impl BuildSpotFleetRequest_SpotMaintenanceStrategies {
    pub fn build(self) -> SpotFleetRequest_SpotMaintenanceStrategies {
        SpotFleetRequest_SpotMaintenanceStrategies {
            capacity_rebalance: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_SpotMaintenanceStrategiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_SpotMaintenanceStrategiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotFleetRequest_SpotMaintenanceStrategiesRef {
        SpotFleetRequest_SpotMaintenanceStrategiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_SpotMaintenanceStrategiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(
        &self,
    ) -> terrars::ListRef<SpotFleetRequest_SpotMaintenanceStrategies_CapacityRebalanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_rebalance", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotFleetRequest_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotFleetRequest_Timeouts {
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

impl terrars::ToListMappable for SpotFleetRequest_Timeouts {
    type O = terrars::BlockAssignable<SpotFleetRequest_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotFleetRequest_Timeouts {}

impl BuildSpotFleetRequest_Timeouts {
    pub fn build(self) -> SpotFleetRequest_Timeouts {
        SpotFleetRequest_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpotFleetRequest_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotFleetRequest_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotFleetRequest_TimeoutsRef {
        SpotFleetRequest_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotFleetRequest_TimeoutsRef {
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
