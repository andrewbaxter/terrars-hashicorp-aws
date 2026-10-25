#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2FleetData_ {
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
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excess_capacity_termination_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfilled_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfilled_on_demand_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_unhealthy_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_instances_with_expiration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_until: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_instance_set: std::option::Option<std::vec::Vec<Ec2Fleet_FleetInstanceSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_config: std::option::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_options: std::option::Option<std::vec::Vec<Ec2Fleet_OnDemandOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_options: std::option::Option<std::vec::Vec<Ec2Fleet_SpotOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_capacity_specification: std::option::Option<std::vec::Vec<Ec2Fleet_TargetCapacitySpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Ec2Fleet_Timeouts>,
    dynamic: Ec2FleetDynamic,
}

struct Ec2Fleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2FleetData_>,
}

#[derive(Clone)]
pub struct Ec2Fleet(std::rc::Rc<Ec2Fleet_>);

impl Ec2Fleet {
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

    #[doc = "Set the field `fleet_state`.\n"]
    pub fn set_fleet_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fleet_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fulfilled_capacity`.\n"]
    pub fn set_fulfilled_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().fulfilled_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fulfilled_on_demand_capacity`.\n"]
    pub fn set_fulfilled_on_demand_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().fulfilled_on_demand_capacity = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `replace_unhealthy_instances`.\n"]
    pub fn set_replace_unhealthy_instances(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replace_unhealthy_instances = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `terminate_instances`.\n"]
    pub fn set_terminate_instances(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terminate_instances_with_expiration`.\n"]
    pub fn set_terminate_instances_with_expiration(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().terminate_instances_with_expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `fleet_instance_set`.\n"]
    pub fn set_fleet_instance_set(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_FleetInstanceSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet_instance_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet_instance_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template_config`.\n"]
    pub fn set_launch_template_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig>>,
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

    #[doc = "Set the field `on_demand_options`.\n"]
    pub fn set_on_demand_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_OnDemandOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_demand_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_demand_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(self, v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_SpotOptions>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spot_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spot_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_capacity_specification`.\n"]
    pub fn set_target_capacity_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_TargetCapacitySpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_capacity_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_capacity_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Ec2Fleet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `fleet_state` after provisioning.\n"]
    pub fn fleet_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfilled_capacity` after provisioning.\n"]
    pub fn fulfilled_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fulfilled_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfilled_on_demand_capacity` after provisioning.\n"]
    pub fn fulfilled_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fulfilled_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instances` after provisioning.\n"]
    pub fn terminate_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminate_instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_instance_set` after provisioning.\n"]
    pub fn fleet_instance_set(&self) -> terrars::ListRef<Ec2Fleet_FleetInstanceSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_instance_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template_config` after provisioning.\n"]
    pub fn launch_template_config(&self) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_options` after provisioning.\n"]
    pub fn on_demand_options(&self) -> terrars::ListRef<Ec2Fleet_OnDemandOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> terrars::ListRef<Ec2Fleet_SpotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_capacity_specification` after provisioning.\n"]
    pub fn target_capacity_specification(&self) -> terrars::ListRef<Ec2Fleet_TargetCapacitySpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_capacity_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2Fleet_TimeoutsRef {
        Ec2Fleet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2Fleet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2Fleet { }

impl terrars::ToListMappable for Ec2Fleet {
    type O = terrars::ListRef<Ec2FleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2Fleet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2Fleet {
    pub tf_id: std::string::String,
}

impl BuildEc2Fleet {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2Fleet {
        let out = Ec2Fleet(std::rc::Rc::new(Ec2Fleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2FleetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                context: core::default::Default::default(),
                excess_capacity_termination_policy: core::default::Default::default(),
                fleet_state: core::default::Default::default(),
                fulfilled_capacity: core::default::Default::default(),
                fulfilled_on_demand_capacity: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replace_unhealthy_instances: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                terminate_instances: core::default::Default::default(),
                terminate_instances_with_expiration: core::default::Default::default(),
                type_: core::default::Default::default(),
                valid_from: core::default::Default::default(),
                valid_until: core::default::Default::default(),
                fleet_instance_set: core::default::Default::default(),
                launch_template_config: core::default::Default::default(),
                on_demand_options: core::default::Default::default(),
                spot_options: core::default::Default::default(),
                target_capacity_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2FleetRef {
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

    #[doc = "Get a reference to the value of field `fleet_state` after provisioning.\n"]
    pub fn fleet_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfilled_capacity` after provisioning.\n"]
    pub fn fulfilled_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fulfilled_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfilled_on_demand_capacity` after provisioning.\n"]
    pub fn fulfilled_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fulfilled_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instances` after provisioning.\n"]
    pub fn terminate_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminate_instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instances_with_expiration` after provisioning.\n"]
    pub fn terminate_instances_with_expiration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instances_with_expiration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_instance_set` after provisioning.\n"]
    pub fn fleet_instance_set(&self) -> terrars::ListRef<Ec2Fleet_FleetInstanceSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_instance_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template_config` after provisioning.\n"]
    pub fn launch_template_config(&self) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_options` after provisioning.\n"]
    pub fn on_demand_options(&self) -> terrars::ListRef<Ec2Fleet_OnDemandOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> terrars::ListRef<Ec2Fleet_SpotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_capacity_specification` after provisioning.\n"]
    pub fn target_capacity_specification(&self) -> terrars::ListRef<Ec2Fleet_TargetCapacitySpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_capacity_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2Fleet_TimeoutsRef {
        Ec2Fleet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2FleetDynamic {
    fleet_instance_set: std::option::Option<terrars::DynamicBlock<Ec2Fleet_FleetInstanceSet>>,
    launch_template_config: std::option::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig>>,
    on_demand_options: std::option::Option<terrars::DynamicBlock<Ec2Fleet_OnDemandOptions>>,
    spot_options: std::option::Option<terrars::DynamicBlock<Ec2Fleet_SpotOptions>>,
    target_capacity_specification: std::option::Option<terrars::DynamicBlock<Ec2Fleet_TargetCapacitySpecification>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_FleetInstanceSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2Fleet_FleetInstanceSet {
    #[doc = "Set the field `instance_ids`.\n"]
    pub fn set_instance_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifecycle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform`.\n"]
    pub fn set_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_FleetInstanceSet {
    type O = terrars::BlockAssignable<Ec2Fleet_FleetInstanceSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_FleetInstanceSet {}

impl BuildEc2Fleet_FleetInstanceSet {
    pub fn build(self) -> Ec2Fleet_FleetInstanceSet {
        Ec2Fleet_FleetInstanceSet {
            instance_ids: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
            platform: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_FleetInstanceSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_FleetInstanceSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_FleetInstanceSetRef {
        Ec2Fleet_FleetInstanceSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_FleetInstanceSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_ids` after provisioning.\n"]
    pub fn instance_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    version: terrars::PrimField<std::string::String>,
}

impl Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
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
}

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildEc2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
        Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
        Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecificationRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCountRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCountRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
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
::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    type O =
        terrars
        ::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    type O =
        terrars
        ::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpuRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpuRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMibRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMibRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {
    type O =
        terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbpsRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {
    type O =
        terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCountRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCountRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {
    type O =
        terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGbRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGbRef {
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCountRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCountRef {
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
struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsDynamic {
    accelerator_count: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount>>,
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib>,
    >,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps>,
    >,
    memory_gib_per_vcpu: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu>>,
    memory_mib: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib>>,
    network_bandwidth_gbps: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps>>,
    network_interface_count: std
    ::option
    ::Option<
        terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount>,
    >,
    total_local_storage_gb: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb>>,
    vcpu_count: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {
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
    max_spot_price_as_percentage_of_optimal_on_demand_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std::option::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std::option::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount>>,
    dynamic: Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsDynamic,
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {
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

    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = std::option::Option::Some(v.into());
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCount,
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMib,
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbps,
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpu,
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
                        ::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMib>,
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbps,
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCount,
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
                            Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGb,
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
                        ::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCount>,
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements {
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
            max_spot_price_as_percentage_of_optimal_on_demand_price: core::default::Default::default(),
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
pub struct Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsRef {
        Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsRef {
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
        "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_spot_price_as_percentage_of_optimal_on_demand_price", self.base),
        )
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
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_AcceleratorTotalMemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_BaselineEbsBandwidthMbpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryGibPerVcpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_MemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkBandwidthGbpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_bandwidth_gbps", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_NetworkInterfaceCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_TotalLocalStorageGbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements_VcpuCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2Fleet_LaunchTemplateConfig_OverrideDynamic {
    instance_requirements: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_LaunchTemplateConfig_Override {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements>>,
    dynamic: Ec2Fleet_LaunchTemplateConfig_OverrideDynamic,
}

impl Ec2Fleet_LaunchTemplateConfig_Override {
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

    #[doc = "Set the field `max_price`.\n"]
    pub fn set_max_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
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
                    ::Into<terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirements>>,
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

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig_Override {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig_Override {}

impl BuildEc2Fleet_LaunchTemplateConfig_Override {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig_Override {
        Ec2Fleet_LaunchTemplateConfig_Override {
            availability_zone: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            max_price: core::default::Default::default(),
            priority: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            weighted_capacity: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfig_OverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfig_OverrideRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_LaunchTemplateConfig_OverrideRef {
        Ec2Fleet_LaunchTemplateConfig_OverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfig_OverrideRef {
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

    #[doc = "Get a reference to the value of field `max_price` after provisioning.\n"]
    pub fn max_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_price", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
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
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_Override_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2Fleet_LaunchTemplateConfigDynamic {
    launch_template_specification: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification>>,
    override_: std::option::Option<terrars::DynamicBlock<Ec2Fleet_LaunchTemplateConfig_Override>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_LaunchTemplateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_specification: std
    ::option
    ::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<std::vec::Vec<Ec2Fleet_LaunchTemplateConfig_Override>>,
    dynamic: Ec2Fleet_LaunchTemplateConfigDynamic,
}

impl Ec2Fleet_LaunchTemplateConfig {
    #[doc = "Set the field `launch_template_specification`.\n"]
    pub fn set_launch_template_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecification>>,
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

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig_Override>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_LaunchTemplateConfig {
    type O = terrars::BlockAssignable<Ec2Fleet_LaunchTemplateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_LaunchTemplateConfig {}

impl BuildEc2Fleet_LaunchTemplateConfig {
    pub fn build(self) -> Ec2Fleet_LaunchTemplateConfig {
        Ec2Fleet_LaunchTemplateConfig {
            launch_template_specification: core::default::Default::default(),
            override_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_LaunchTemplateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_LaunchTemplateConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_LaunchTemplateConfigRef {
        Ec2Fleet_LaunchTemplateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_LaunchTemplateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_specification` after provisioning.\n"]
    pub fn launch_template_specification(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_LaunchTemplateSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::ListRef<Ec2Fleet_LaunchTemplateConfig_OverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2Fleet_OnDemandOptions_CapacityReservationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2Fleet_OnDemandOptions_CapacityReservationOptions {
    #[doc = "Set the field `usage_strategy`.\n"]
    pub fn set_usage_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_OnDemandOptions_CapacityReservationOptions {
    type O = terrars::BlockAssignable<Ec2Fleet_OnDemandOptions_CapacityReservationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_OnDemandOptions_CapacityReservationOptions {}

impl BuildEc2Fleet_OnDemandOptions_CapacityReservationOptions {
    pub fn build(self) -> Ec2Fleet_OnDemandOptions_CapacityReservationOptions {
        Ec2Fleet_OnDemandOptions_CapacityReservationOptions { usage_strategy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_OnDemandOptions_CapacityReservationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_OnDemandOptions_CapacityReservationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_OnDemandOptions_CapacityReservationOptionsRef {
        Ec2Fleet_OnDemandOptions_CapacityReservationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_OnDemandOptions_CapacityReservationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `usage_strategy` after provisioning.\n"]
    pub fn usage_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2Fleet_OnDemandOptionsDynamic {
    capacity_reservation_options: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_OnDemandOptions_CapacityReservationOptions>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_OnDemandOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_total_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_target_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_availability_zone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_instance_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_options: std::option::Option<std::vec::Vec<Ec2Fleet_OnDemandOptions_CapacityReservationOptions>>,
    dynamic: Ec2Fleet_OnDemandOptionsDynamic,
}

impl Ec2Fleet_OnDemandOptions {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_total_price`.\n"]
    pub fn set_max_total_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_total_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_target_capacity`.\n"]
    pub fn set_min_target_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_target_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_availability_zone`.\n"]
    pub fn set_single_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.single_availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_instance_type`.\n"]
    pub fn set_single_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.single_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_options`.\n"]
    pub fn set_capacity_reservation_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_OnDemandOptions_CapacityReservationOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_reservation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_OnDemandOptions {
    type O = terrars::BlockAssignable<Ec2Fleet_OnDemandOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_OnDemandOptions {}

impl BuildEc2Fleet_OnDemandOptions {
    pub fn build(self) -> Ec2Fleet_OnDemandOptions {
        Ec2Fleet_OnDemandOptions {
            allocation_strategy: core::default::Default::default(),
            max_total_price: core::default::Default::default(),
            min_target_capacity: core::default::Default::default(),
            single_availability_zone: core::default::Default::default(),
            single_instance_type: core::default::Default::default(),
            capacity_reservation_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_OnDemandOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_OnDemandOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_OnDemandOptionsRef {
        Ec2Fleet_OnDemandOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_OnDemandOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_total_price` after provisioning.\n"]
    pub fn max_total_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_total_price", self.base))
    }

    #[doc = "Get a reference to the value of field `min_target_capacity` after provisioning.\n"]
    pub fn min_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_target_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `single_availability_zone` after provisioning.\n"]
    pub fn single_availability_zone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `single_instance_type` after provisioning.\n"]
    pub fn single_instance_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_options` after provisioning.\n"]
    pub fn capacity_reservation_options(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_OnDemandOptions_CapacityReservationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_options", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_delay: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {
    #[doc = "Set the field `replacement_strategy`.\n"]
    pub fn set_replacement_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replacement_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `termination_delay`.\n"]
    pub fn set_termination_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.termination_delay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {
    type O = terrars::BlockAssignable<Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {}

impl BuildEc2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {
    pub fn build(self) -> Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {
        Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance {
            replacement_strategy: core::default::Default::default(),
            termination_delay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalanceRef {
        Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replacement_strategy` after provisioning.\n"]
    pub fn replacement_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `termination_delay` after provisioning.\n"]
    pub fn termination_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2Fleet_SpotOptions_MaintenanceStrategiesDynamic {
    capacity_rebalance: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_SpotOptions_MaintenanceStrategies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_rebalance: std::option::Option<std::vec::Vec<Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance>>,
    dynamic: Ec2Fleet_SpotOptions_MaintenanceStrategiesDynamic,
}

impl Ec2Fleet_SpotOptions_MaintenanceStrategies {
    #[doc = "Set the field `capacity_rebalance`.\n"]
    pub fn set_capacity_rebalance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalance>>,
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

impl terrars::ToListMappable for Ec2Fleet_SpotOptions_MaintenanceStrategies {
    type O = terrars::BlockAssignable<Ec2Fleet_SpotOptions_MaintenanceStrategies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_SpotOptions_MaintenanceStrategies {}

impl BuildEc2Fleet_SpotOptions_MaintenanceStrategies {
    pub fn build(self) -> Ec2Fleet_SpotOptions_MaintenanceStrategies {
        Ec2Fleet_SpotOptions_MaintenanceStrategies {
            capacity_rebalance: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_SpotOptions_MaintenanceStrategiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_SpotOptions_MaintenanceStrategiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_SpotOptions_MaintenanceStrategiesRef {
        Ec2Fleet_SpotOptions_MaintenanceStrategiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_SpotOptions_MaintenanceStrategiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_rebalance` after provisioning.\n"]
    pub fn capacity_rebalance(
        &self,
    ) -> terrars::ListRef<Ec2Fleet_SpotOptions_MaintenanceStrategies_CapacityRebalanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_rebalance", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2Fleet_SpotOptionsDynamic {
    maintenance_strategies: std::option::Option<terrars::DynamicBlock<Ec2Fleet_SpotOptions_MaintenanceStrategies>>,
}

#[derive(Serialize)]
pub struct Ec2Fleet_SpotOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_interruption_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_pools_to_use_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_total_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_target_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_availability_zone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_instance_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_strategies: std::option::Option<std::vec::Vec<Ec2Fleet_SpotOptions_MaintenanceStrategies>>,
    dynamic: Ec2Fleet_SpotOptionsDynamic,
}

impl Ec2Fleet_SpotOptions {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_interruption_behavior`.\n"]
    pub fn set_instance_interruption_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_interruption_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_pools_to_use_count`.\n"]
    pub fn set_instance_pools_to_use_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_pools_to_use_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_total_price`.\n"]
    pub fn set_max_total_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_total_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_target_capacity`.\n"]
    pub fn set_min_target_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_target_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_availability_zone`.\n"]
    pub fn set_single_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.single_availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_instance_type`.\n"]
    pub fn set_single_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.single_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_strategies`.\n"]
    pub fn set_maintenance_strategies(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2Fleet_SpotOptions_MaintenanceStrategies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_strategies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_strategies = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_SpotOptions {
    type O = terrars::BlockAssignable<Ec2Fleet_SpotOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_SpotOptions {}

impl BuildEc2Fleet_SpotOptions {
    pub fn build(self) -> Ec2Fleet_SpotOptions {
        Ec2Fleet_SpotOptions {
            allocation_strategy: core::default::Default::default(),
            instance_interruption_behavior: core::default::Default::default(),
            instance_pools_to_use_count: core::default::Default::default(),
            max_total_price: core::default::Default::default(),
            min_target_capacity: core::default::Default::default(),
            single_availability_zone: core::default::Default::default(),
            single_instance_type: core::default::Default::default(),
            maintenance_strategies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_SpotOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_SpotOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_SpotOptionsRef {
        Ec2Fleet_SpotOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_SpotOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_interruption_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_pools_to_use_count` after provisioning.\n"]
    pub fn instance_pools_to_use_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_pools_to_use_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_total_price` after provisioning.\n"]
    pub fn max_total_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_total_price", self.base))
    }

    #[doc = "Get a reference to the value of field `min_target_capacity` after provisioning.\n"]
    pub fn min_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_target_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `single_availability_zone` after provisioning.\n"]
    pub fn single_availability_zone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `single_instance_type` after provisioning.\n"]
    pub fn single_instance_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_strategies` after provisioning.\n"]
    pub fn maintenance_strategies(&self) -> terrars::ListRef<Ec2Fleet_SpotOptions_MaintenanceStrategiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_strategies", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2Fleet_TargetCapacitySpecification {
    default_target_capacity_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_target_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_target_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_capacity_unit_type: std::option::Option<terrars::PrimField<std::string::String>>,
    total_target_capacity: terrars::PrimField<f64>,
}

impl Ec2Fleet_TargetCapacitySpecification {
    #[doc = "Set the field `on_demand_target_capacity`.\n"]
    pub fn set_on_demand_target_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.on_demand_target_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_target_capacity`.\n"]
    pub fn set_spot_target_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.spot_target_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_capacity_unit_type`.\n"]
    pub fn set_target_capacity_unit_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_capacity_unit_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2Fleet_TargetCapacitySpecification {
    type O = terrars::BlockAssignable<Ec2Fleet_TargetCapacitySpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_TargetCapacitySpecification {
    #[doc = ""]
    pub default_target_capacity_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub total_target_capacity: terrars::PrimField<f64>,
}

impl BuildEc2Fleet_TargetCapacitySpecification {
    pub fn build(self) -> Ec2Fleet_TargetCapacitySpecification {
        Ec2Fleet_TargetCapacitySpecification {
            default_target_capacity_type: self.default_target_capacity_type,
            on_demand_target_capacity: core::default::Default::default(),
            spot_target_capacity: core::default::Default::default(),
            target_capacity_unit_type: core::default::Default::default(),
            total_target_capacity: self.total_target_capacity,
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_TargetCapacitySpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_TargetCapacitySpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_TargetCapacitySpecificationRef {
        Ec2Fleet_TargetCapacitySpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_TargetCapacitySpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_target_capacity_type` after provisioning.\n"]
    pub fn default_target_capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_target_capacity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `on_demand_target_capacity` after provisioning.\n"]
    pub fn on_demand_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_target_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_target_capacity` after provisioning.\n"]
    pub fn spot_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_target_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_capacity_unit_type` after provisioning.\n"]
    pub fn target_capacity_unit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_capacity_unit_type", self.base))
    }

    #[doc = "Get a reference to the value of field `total_target_capacity` after provisioning.\n"]
    pub fn total_target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_target_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2Fleet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2Fleet_Timeouts {
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

impl terrars::ToListMappable for Ec2Fleet_Timeouts {
    type O = terrars::BlockAssignable<Ec2Fleet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2Fleet_Timeouts {}

impl BuildEc2Fleet_Timeouts {
    pub fn build(self) -> Ec2Fleet_Timeouts {
        Ec2Fleet_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2Fleet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2Fleet_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2Fleet_TimeoutsRef {
        Ec2Fleet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2Fleet_TimeoutsRef {
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
