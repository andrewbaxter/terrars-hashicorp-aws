#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ArcregionswitchPlanData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_region: std::option::Option<terrars::PrimField<std::string::String>>,
    recovery_approach: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_time_objective_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    regions: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_alarms: std::option::Option<std::vec::Vec<ArcregionswitchPlan_AssociatedAlarms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_configuration: std::option::Option<std::vec::Vec<ArcregionswitchPlan_ReportConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ArcregionswitchPlan_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Triggers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow>>,
    dynamic: ArcregionswitchPlanDynamic,
}

struct ArcregionswitchPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ArcregionswitchPlanData_>,
}

#[derive(Clone)]
pub struct ArcregionswitchPlan(std::rc::Rc<ArcregionswitchPlan_>);

impl ArcregionswitchPlan {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_region`.\n"]
    pub fn set_primary_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().primary_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_time_objective_minutes`.\n"]
    pub fn set_recovery_time_objective_minutes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().recovery_time_objective_minutes = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `associated_alarms`.\n"]
    pub fn set_associated_alarms(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_AssociatedAlarms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().associated_alarms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.associated_alarms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `report_configuration`.\n"]
    pub fn set_report_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_ReportConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().report_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.report_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ArcregionswitchPlan_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Triggers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().triggers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.triggers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow`.\n"]
    pub fn set_workflow(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workflow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workflow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_region` after provisioning.\n"]
    pub fn primary_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recovery_approach` after provisioning.\n"]
    pub fn recovery_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recovery_approach", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recovery_time_objective_minutes` after provisioning.\n"]
    pub fn recovery_time_objective_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.recovery_time_objective_minutes", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_configuration` after provisioning.\n"]
    pub fn report_configuration(&self) -> terrars::ListRef<ArcregionswitchPlan_ReportConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArcregionswitchPlan_TimeoutsRef {
        ArcregionswitchPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::ListRef<ArcregionswitchPlan_TriggersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> terrars::ListRef<ArcregionswitchPlan_WorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow", self.extract_ref()))
    }
}

impl terrars::Referable for ArcregionswitchPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ArcregionswitchPlan { }

impl terrars::ToListMappable for ArcregionswitchPlan {
    type O = terrars::ListRef<ArcregionswitchPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ArcregionswitchPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_arcregionswitch_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildArcregionswitchPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub recovery_approach: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub regions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> ArcregionswitchPlan {
        let out = ArcregionswitchPlan(std::rc::Rc::new(ArcregionswitchPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ArcregionswitchPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role: self.execution_role,
                name: self.name,
                primary_region: core::default::Default::default(),
                recovery_approach: self.recovery_approach,
                recovery_time_objective_minutes: core::default::Default::default(),
                region: core::default::Default::default(),
                regions: self.regions,
                tags: core::default::Default::default(),
                associated_alarms: core::default::Default::default(),
                report_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                triggers: core::default::Default::default(),
                workflow: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ArcregionswitchPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ArcregionswitchPlanRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_region` after provisioning.\n"]
    pub fn primary_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recovery_approach` after provisioning.\n"]
    pub fn recovery_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recovery_approach", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recovery_time_objective_minutes` after provisioning.\n"]
    pub fn recovery_time_objective_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.recovery_time_objective_minutes", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_configuration` after provisioning.\n"]
    pub fn report_configuration(&self) -> terrars::ListRef<ArcregionswitchPlan_ReportConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArcregionswitchPlan_TimeoutsRef {
        ArcregionswitchPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> terrars::ListRef<ArcregionswitchPlan_TriggersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> terrars::ListRef<ArcregionswitchPlan_WorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlanDynamic {
    associated_alarms: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_AssociatedAlarms>>,
    report_configuration: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_ReportConfiguration>>,
    triggers: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_Triggers>>,
    workflow: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_AssociatedAlarms {
    alarm_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    map_block_key: terrars::PrimField<std::string::String>,
    resource_identifier: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_AssociatedAlarms {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_AssociatedAlarms {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_AssociatedAlarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_AssociatedAlarms {
    #[doc = ""]
    pub alarm_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub map_block_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_identifier: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_AssociatedAlarms {
    pub fn build(self) -> ArcregionswitchPlan_AssociatedAlarms {
        ArcregionswitchPlan_AssociatedAlarms {
            alarm_type: self.alarm_type,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            map_block_key: self.map_block_key,
            resource_identifier: self.resource_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_AssociatedAlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_AssociatedAlarmsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_AssociatedAlarmsRef {
        ArcregionswitchPlan_AssociatedAlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_AssociatedAlarmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_type` after provisioning.\n"]
    pub fn alarm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_block_key", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration {
    bucket_owner: terrars::PrimField<std::string::String>,
    bucket_path: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration { }

impl terrars::ToListMappable for ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration {
    #[doc = ""]
    pub bucket_owner: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bucket_path: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration {
    pub fn build(self) -> ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration {
        ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration {
            bucket_owner: self.bucket_owner,
            bucket_path: self.bucket_path,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3ConfigurationRef {
        ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_path` after provisioning.\n"]
    pub fn bucket_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_ReportConfiguration_ReportOutputDynamic {
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_ReportConfiguration_ReportOutput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration>>,
    dynamic: ArcregionswitchPlan_ReportConfiguration_ReportOutputDynamic,
}

impl ArcregionswitchPlan_ReportConfiguration_ReportOutput {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_ReportConfiguration_ReportOutput {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_ReportConfiguration_ReportOutput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_ReportConfiguration_ReportOutput {}

impl BuildArcregionswitchPlan_ReportConfiguration_ReportOutput {
    pub fn build(self) -> ArcregionswitchPlan_ReportConfiguration_ReportOutput {
        ArcregionswitchPlan_ReportConfiguration_ReportOutput {
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_ReportConfiguration_ReportOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_ReportConfiguration_ReportOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_ReportConfiguration_ReportOutputRef {
        ArcregionswitchPlan_ReportConfiguration_ReportOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_ReportConfiguration_ReportOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_ReportConfiguration_ReportOutput_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_ReportConfigurationDynamic {
    report_output: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_ReportConfiguration_ReportOutput>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_ReportConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_output: std::option::Option<std::vec::Vec<ArcregionswitchPlan_ReportConfiguration_ReportOutput>>,
    dynamic: ArcregionswitchPlan_ReportConfigurationDynamic,
}

impl ArcregionswitchPlan_ReportConfiguration {
    #[doc = "Set the field `report_output`.\n"]
    pub fn set_report_output(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_ReportConfiguration_ReportOutput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.report_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.report_output = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_ReportConfiguration {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_ReportConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_ReportConfiguration {}

impl BuildArcregionswitchPlan_ReportConfiguration {
    pub fn build(self) -> ArcregionswitchPlan_ReportConfiguration {
        ArcregionswitchPlan_ReportConfiguration {
            report_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_ReportConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_ReportConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_ReportConfigurationRef {
        ArcregionswitchPlan_ReportConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_ReportConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `report_output` after provisioning.\n"]
    pub fn report_output(&self) -> terrars::ListRef<ArcregionswitchPlan_ReportConfiguration_ReportOutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_output", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArcregionswitchPlan_Timeouts {
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

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Timeouts {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Timeouts {}

impl BuildArcregionswitchPlan_Timeouts {
    pub fn build(self) -> ArcregionswitchPlan_Timeouts {
        ArcregionswitchPlan_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_TimeoutsRef {
        ArcregionswitchPlan_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_TimeoutsRef {
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

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Triggers_Conditions {
    associated_alarm_name: terrars::PrimField<std::string::String>,
    condition: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Triggers_Conditions { }

impl terrars::ToListMappable for ArcregionswitchPlan_Triggers_Conditions {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Triggers_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Triggers_Conditions {
    #[doc = ""]
    pub associated_alarm_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub condition: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Triggers_Conditions {
    pub fn build(self) -> ArcregionswitchPlan_Triggers_Conditions {
        ArcregionswitchPlan_Triggers_Conditions {
            associated_alarm_name: self.associated_alarm_name,
            condition: self.condition,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Triggers_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Triggers_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_Triggers_ConditionsRef {
        ArcregionswitchPlan_Triggers_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Triggers_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `associated_alarm_name` after provisioning.\n"]
    pub fn associated_alarm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_alarm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_TriggersDynamic {
    conditions: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_Triggers_Conditions>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Triggers {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    min_delay_minutes_between_executions: terrars::PrimField<f64>,
    target_region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Triggers_Conditions>>,
    dynamic: ArcregionswitchPlan_TriggersDynamic,
}

impl ArcregionswitchPlan_Triggers {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Triggers_Conditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Triggers {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Triggers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Triggers {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub min_delay_minutes_between_executions: terrars::PrimField<f64>,
    #[doc = ""]
    pub target_region: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Triggers {
    pub fn build(self) -> ArcregionswitchPlan_Triggers {
        ArcregionswitchPlan_Triggers {
            action: self.action,
            description: core::default::Default::default(),
            min_delay_minutes_between_executions: self.min_delay_minutes_between_executions,
            target_region: self.target_region,
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_TriggersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_TriggersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_TriggersRef {
        ArcregionswitchPlan_TriggersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_TriggersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `min_delay_minutes_between_executions` after provisioning.\n"]
    pub fn min_delay_minutes_between_executions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_delay_minutes_between_executions", self.base))
    }

    #[doc = "Get a reference to the value of field `target_region` after provisioning.\n"]
    pub fn target_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_region", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<ArcregionswitchPlan_Triggers_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl {
    routing_control_arn: terrars::PrimField<std::string::String>,
    state: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl { }

impl terrars
::ToListMappable for ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl {
    type O =
        terrars
        ::BlockAssignable<
            ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl {
    #[doc = ""]
    pub routing_control_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl {
    pub fn build(
        self,
    ) -> ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl {
        ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl {
            routing_control_arn: self.routing_control_arn,
            state: self.state,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControlRef {
        ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routing_control_arn` after provisioning.\n"]
    pub fn routing_control_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_control_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsDynamic {
    routing_control: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_control: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl>,
    >,
    dynamic: ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
    #[doc = "Set the field `routing_control`.\n"]
    pub fn set_routing_control(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routing_control = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routing_control = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
    type O =
        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
        ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls {
            region: self.region,
            routing_control: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsRef {
        ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_control` after provisioning.\n"]
    pub fn routing_control(
        &self,
    ) -> terrars
    ::ListRef<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls_RoutingControlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_control", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigDynamic {
    region_and_routing_controls: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls>,
    >,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_and_routing_controls: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_and_routing_controls`.\n"]
    pub fn set_region_and_routing_controls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig_RegionAndRoutingControls,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region_and_routing_controls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region_and_routing_controls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {}

impl BuildArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {
        ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig {
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
            region_and_routing_controls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef {
        ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    global_cluster_identifier: terrars::PrimField<std::string::String>,
    instance_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
    region_database_cluster_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
    #[doc = ""]
    pub global_cluster_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub region_database_cluster_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
        ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig {
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            global_cluster_identifier: self.global_cluster_identifier,
            instance_arns: self.instance_arns,
            region_database_cluster_arns: self.region_database_cluster_arns,
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef {
        ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_arns` after provisioning.\n"]
    pub fn instance_arns(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instance_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `region_database_cluster_arns` after provisioning.\n"]
    pub fn region_database_cluster_arns(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_database_cluster_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    global_cluster_identifier: terrars::PrimField<std::string::String>,
    region_database_cluster_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_percent`.\n"]
    pub fn set_target_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
    #[doc = ""]
    pub global_cluster_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region_database_cluster_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
        ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig {
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            global_cluster_identifier: self.global_cluster_identifier,
            region_database_cluster_arns: self.region_database_cluster_arns,
            target_percent: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef {
        ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `region_database_cluster_arns` after provisioning.\n"]
    pub fn region_database_cluster_arns(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_database_cluster_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `target_percent` after provisioning.\n"]
    pub fn target_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
        ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda {
            arn: self.arn,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_LambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_LambdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_LambdaRef {
        ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_LambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_LambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful {
    behavior: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful { }

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful {
        ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful { behavior: self.behavior }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef {
        ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigDynamic {
    lambda: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda>>,
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
    region_to_run: terrars::PrimField<std::string::String>,
    retry_interval_minutes: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Lambda>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
    #[doc = ""]
    pub region_to_run: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub retry_interval_minutes: terrars::PrimField<f64>,
}

impl BuildArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
        ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig {
            region_to_run: self.region_to_run,
            retry_interval_minutes: self.retry_interval_minutes,
            timeout_minutes: core::default::Default::default(),
            lambda: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef {
        ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region_to_run` after provisioning.\n"]
    pub fn region_to_run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_to_run", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_interval_minutes` after provisioning.\n"]
    pub fn retry_interval_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_interval_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_LambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful {
    ungraceful: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful { }

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful {
    #[doc = ""]
    pub ungraceful: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful {
        ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful { ungraceful: self.ungraceful }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef {
        ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_DocumentDbConfigDynamic {
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
    behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    database_cluster_arns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    global_cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_DocumentDbConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_cluster_arns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub global_cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
        ArcregionswitchPlan_Workflow_Step_DocumentDbConfig {
            behavior: self.behavior,
            cross_account_role: core::default::Default::default(),
            database_cluster_arns: self.database_cluster_arns,
            external_id: core::default::Default::default(),
            global_cluster_identifier: self.global_cluster_identifier,
            timeout_minutes: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef {
        ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `database_cluster_arns` after provisioning.\n"]
    pub fn database_cluster_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_cluster_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
        ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg {
            arn: self.arn,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_AsgRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_AsgRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_AsgRef {
        ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_AsgRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_AsgRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful {
    minimum_success_percentage: terrars::PrimField<f64>,
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful { }

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful {
    #[doc = ""]
    pub minimum_success_percentage: terrars::PrimField<f64>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful {
            minimum_success_percentage: self.minimum_success_percentage,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_success_percentage` after provisioning.\n"]
    pub fn minimum_success_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_success_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigDynamic {
    asg: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg>>,
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
    capacity_monitoring_approach: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asg: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
    #[doc = "Set the field `target_percent`.\n"]
    pub fn set_target_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asg`.\n"]
    pub fn set_asg(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_Asg>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asg = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asg = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
    #[doc = ""]
    pub capacity_monitoring_approach: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
        ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig {
            capacity_monitoring_approach: self.capacity_monitoring_approach,
            target_percent: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
            asg: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef {
        ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_monitoring_approach` after provisioning.\n"]
    pub fn capacity_monitoring_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_monitoring_approach", self.base))
    }

    #[doc = "Get a reference to the value of field `target_percent` after provisioning.\n"]
    pub fn target_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `asg` after provisioning.\n"]
    pub fn asg(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig_AsgRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asg", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    service_arn: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_arn: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
        ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service {
            cluster_arn: self.cluster_arn,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            service_arn: self.service_arn,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_ServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_ServiceRef {
        ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_ServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_ServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigDynamic {
    service: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service>>,
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
    capacity_monitoring_approach: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
    #[doc = "Set the field `target_percent`.\n"]
    pub fn set_target_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_Service>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
    #[doc = ""]
    pub capacity_monitoring_approach: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
        ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig {
            capacity_monitoring_approach: self.capacity_monitoring_approach,
            target_percent: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
            service: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef {
        ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_monitoring_approach` after provisioning.\n"]
    pub fn capacity_monitoring_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_monitoring_approach", self.base))
    }

    #[doc = "Get a reference to the value of field `target_percent` after provisioning.\n"]
    pub fn target_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig_ServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters {
            cluster_arn: self.cluster_arn,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClustersRef {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType {
    api_version: terrars::PrimField<std::string::String>,
    kind: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType { }

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType {
    type O =
        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType {
    #[doc = ""]
    pub api_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub kind: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType {
            api_version: self.api_version,
            kind: self.kind,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceTypeRef {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hpa_name: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    resource_name: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
    #[doc = "Set the field `hpa_name`.\n"]
    pub fn set_hpa_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hpa_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
    type O =
        terrars
        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_name: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources {
            hpa_name: core::default::Default::default(),
            name: self.name,
            namespace: self.namespace,
            resource_name: self.resource_name,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_ResourcesRef {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hpa_name` after provisioning.\n"]
    pub fn hpa_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hpa_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\n"]
    pub fn resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesDynamic {
    resources: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources>,
    >,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources_Resources,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources {
            namespace: self.namespace,
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesRef {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigDynamic {
    eks_clusters: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters>>,
    kubernetes_resource_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType>,
    >,
    scaling_resources: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources>>,
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
    capacity_monitoring_approach: terrars::PrimField<std::string::String>,
    target_percent: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_clusters: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_resource_type: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_resources: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eks_clusters`.\n"]
    pub fn set_eks_clusters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClusters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eks_clusters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eks_clusters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes_resource_type`.\n"]
    pub fn set_kubernetes_resource_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubernetes_resource_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_resource_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_resources`.\n"]
    pub fn set_scaling_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResources>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scaling_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scaling_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_Ungraceful>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
    #[doc = ""]
    pub capacity_monitoring_approach: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_percent: terrars::PrimField<f64>,
}

impl BuildArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig {
            capacity_monitoring_approach: self.capacity_monitoring_approach,
            target_percent: self.target_percent,
            timeout_minutes: core::default::Default::default(),
            eks_clusters: core::default::Default::default(),
            kubernetes_resource_type: core::default::Default::default(),
            scaling_resources: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef {
        ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_monitoring_approach` after provisioning.\n"]
    pub fn capacity_monitoring_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_monitoring_approach", self.base))
    }

    #[doc = "Get a reference to the value of field `target_percent` after provisioning.\n"]
    pub fn target_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `eks_clusters` after provisioning.\n"]
    pub fn eks_clusters(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_EksClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_clusters", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes_resource_type` after provisioning.\n"]
    pub fn kubernetes_resource_type(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_KubernetesResourceTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_resources` after provisioning.\n"]
    pub fn scaling_resources(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_ScalingResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
    approval_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
    #[doc = ""]
    pub approval_role: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
        ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig {
            approval_role: self.approval_role,
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef {
        ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approval_role` after provisioning.\n"]
    pub fn approval_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_role", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigDynamic {
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
    behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    database_cluster_arns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    global_cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_cluster_arns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub global_cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
        ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig {
            behavior: self.behavior,
            cross_account_role: core::default::Default::default(),
            database_cluster_arns: self.database_cluster_arns,
            external_id: core::default::Default::default(),
            global_cluster_identifier: self.global_cluster_identifier,
            timeout_minutes: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef {
        ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `database_cluster_arns` after provisioning.\n"]
    pub fn database_cluster_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_cluster_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
    type O =
        terrars
        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
        ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping {
            arn: self.arn,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            region: self.region,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMappingRef {
        ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigDynamic {
    region_event_source_mapping: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping>,
    >,
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_event_source_mapping: std
    ::option
    ::Option<
        std::vec::Vec<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_event_source_mapping`.\n"]
    pub fn set_region_event_source_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig_RegionEventSourceMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region_event_source_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region_event_source_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_Ungraceful>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
        ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig {
            action: self.action,
            timeout_minutes: core::default::Default::default(),
            region_event_source_mapping: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef {
        ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigDynamic {
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
    behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    global_cluster_identifier: terrars::PrimField<std::string::String>,
    region_database_cluster_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub global_cluster_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region_database_cluster_arns: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
        ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig {
            behavior: self.behavior,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            global_cluster_identifier: self.global_cluster_identifier,
            region_database_cluster_arns: self.region_database_cluster_arns,
            timeout_minutes: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef {
        ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `global_cluster_identifier` after provisioning.\n"]
    pub fn global_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `region_database_cluster_arns` after provisioning.\n"]
    pub fn region_database_cluster_arns(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_database_cluster_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    db_instance_arn_map: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
    #[doc = ""]
    pub db_instance_arn_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
        ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig {
            cross_account_role: core::default::Default::default(),
            db_instance_arn_map: self.db_instance_arn_map,
            external_id: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef {
        ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `db_instance_arn_map` after provisioning.\n"]
    pub fn db_instance_arn_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.db_instance_arn_map", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    db_instance_arn_map: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
    #[doc = ""]
    pub db_instance_arn_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
        ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig {
            cross_account_role: core::default::Default::default(),
            db_instance_arn_map: self.db_instance_arn_map,
            external_id: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef {
        ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `db_instance_arn_map` after provisioning.\n"]
    pub fn db_instance_arn_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.db_instance_arn_map", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigDynamic {
    ungraceful: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    db_instance_arn_map: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ungraceful: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ungraceful`.\n"]
    pub fn set_ungraceful(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_Ungraceful>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ungraceful = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ungraceful = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
    #[doc = ""]
    pub db_instance_arn_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
        ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig {
            cross_account_role: core::default::Default::default(),
            db_instance_arn_map: self.db_instance_arn_map,
            external_id: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
            ungraceful: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef {
        ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `db_instance_arn_map` after provisioning.\n"]
    pub fn db_instance_arn_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.db_instance_arn_map", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `ungraceful` after provisioning.\n"]
    pub fn ungraceful(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig_UngracefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ungraceful", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
        ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig {
            arn: self.arn,
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef {
        ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet {
    record_set_identifier: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
}

impl ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet { }

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet {
    #[doc = ""]
    pub record_set_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet {
        ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet {
            record_set_identifier: self.record_set_identifier,
            region: self.region,
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSetRef {
        ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_set_identifier` after provisioning.\n"]
    pub fn record_set_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_set_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigDynamic {
    record_set: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    hosted_zone_id: terrars::PrimField<std::string::String>,
    record_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_set: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_set`.\n"]
    pub fn set_record_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSet>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
    #[doc = ""]
    pub hosted_zone_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_name: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
        ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig {
            cross_account_role: core::default::Default::default(),
            external_id: core::default::Default::default(),
            hosted_zone_id: self.hosted_zone_id,
            record_name: self.record_name,
            timeout_minutes: core::default::Default::default(),
            record_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef {
        ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }

    #[doc = "Get a reference to the value of field `record_name` after provisioning.\n"]
    pub fn record_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_name", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `record_set` after provisioning.\n"]
    pub fn record_set(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig_RecordSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepDynamic {
    arc_routing_control_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>>,
    aurora_provisioned_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>>,
    aurora_serverless_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>>,
    custom_action_lambda_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>>,
    document_db_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>>,
    ec2_asg_capacity_increase_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>>,
    ecs_capacity_increase_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>>,
    eks_resource_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>>,
    execution_approval_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>>,
    global_aurora_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>>,
    lambda_event_source_mapping_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>>,
    neptune_global_database_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>>,
    rds_create_cross_region_read_replica_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>>,
    rds_promote_read_replica_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>>,
    rds_switchover_read_replica_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>>,
    region_switch_plan_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>>,
    route53_health_check_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_block_type: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arc_routing_control_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora_provisioned_scaling_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora_serverless_scaling_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_action_lambda_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_db_config: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_asg_capacity_increase_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_capacity_increase_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_resource_scaling_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_approval_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_aurora_config: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_event_source_mapping_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    neptune_global_database_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_create_cross_region_read_replica_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_promote_read_replica_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_switchover_read_replica_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_switch_plan_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route53_health_check_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arc_routing_control_config`.\n"]
    pub fn set_arc_routing_control_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.arc_routing_control_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.arc_routing_control_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aurora_provisioned_scaling_config`.\n"]
    pub fn set_aurora_provisioned_scaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aurora_provisioned_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora_provisioned_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aurora_serverless_scaling_config`.\n"]
    pub fn set_aurora_serverless_scaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aurora_serverless_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora_serverless_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_action_lambda_config`.\n"]
    pub fn set_custom_action_lambda_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_action_lambda_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_action_lambda_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_db_config`.\n"]
    pub fn set_document_db_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.document_db_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.document_db_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_asg_capacity_increase_config`.\n"]
    pub fn set_ec2_asg_capacity_increase_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_asg_capacity_increase_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_asg_capacity_increase_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_capacity_increase_config`.\n"]
    pub fn set_ecs_capacity_increase_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecs_capacity_increase_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecs_capacity_increase_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eks_resource_scaling_config`.\n"]
    pub fn set_eks_resource_scaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eks_resource_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eks_resource_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_approval_config`.\n"]
    pub fn set_execution_approval_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_approval_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_approval_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `global_aurora_config`.\n"]
    pub fn set_global_aurora_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.global_aurora_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.global_aurora_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_event_source_mapping_config`.\n"]
    pub fn set_lambda_event_source_mapping_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_event_source_mapping_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_event_source_mapping_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `neptune_global_database_config`.\n"]
    pub fn set_neptune_global_database_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.neptune_global_database_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.neptune_global_database_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_create_cross_region_read_replica_config`.\n"]
    pub fn set_rds_create_cross_region_read_replica_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_create_cross_region_read_replica_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_create_cross_region_read_replica_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_promote_read_replica_config`.\n"]
    pub fn set_rds_promote_read_replica_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_promote_read_replica_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_promote_read_replica_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_switchover_read_replica_config`.\n"]
    pub fn set_rds_switchover_read_replica_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_switchover_read_replica_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_switchover_read_replica_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region_switch_plan_config`.\n"]
    pub fn set_region_switch_plan_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region_switch_plan_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region_switch_plan_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route53_health_check_config`.\n"]
    pub fn set_route53_health_check_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route53_health_check_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route53_health_check_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
    #[doc = ""]
    pub execution_block_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
        ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step {
            description: core::default::Default::default(),
            execution_block_type: self.execution_block_type,
            name: self.name,
            arc_routing_control_config: core::default::Default::default(),
            aurora_provisioned_scaling_config: core::default::Default::default(),
            aurora_serverless_scaling_config: core::default::Default::default(),
            custom_action_lambda_config: core::default::Default::default(),
            document_db_config: core::default::Default::default(),
            ec2_asg_capacity_increase_config: core::default::Default::default(),
            ecs_capacity_increase_config: core::default::Default::default(),
            eks_resource_scaling_config: core::default::Default::default(),
            execution_approval_config: core::default::Default::default(),
            global_aurora_config: core::default::Default::default(),
            lambda_event_source_mapping_config: core::default::Default::default(),
            neptune_global_database_config: core::default::Default::default(),
            rds_create_cross_region_read_replica_config: core::default::Default::default(),
            rds_promote_read_replica_config: core::default::Default::default(),
            rds_switchover_read_replica_config: core::default::Default::default(),
            region_switch_plan_config: core::default::Default::default(),
            route53_health_check_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepRef {
        ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_block_type` after provisioning.\n"]
    pub fn execution_block_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_block_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `arc_routing_control_config` after provisioning.\n"]
    pub fn arc_routing_control_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arc_routing_control_config", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora_provisioned_scaling_config` after provisioning.\n"]
    pub fn aurora_provisioned_scaling_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora_provisioned_scaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora_serverless_scaling_config` after provisioning.\n"]
    pub fn aurora_serverless_scaling_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora_serverless_scaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_action_lambda_config` after provisioning.\n"]
    pub fn custom_action_lambda_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_action_lambda_config", self.base))
    }

    #[doc = "Get a reference to the value of field `document_db_config` after provisioning.\n"]
    pub fn document_db_config(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_db_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ec2_asg_capacity_increase_config` after provisioning.\n"]
    pub fn ec2_asg_capacity_increase_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_asg_capacity_increase_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ecs_capacity_increase_config` after provisioning.\n"]
    pub fn ecs_capacity_increase_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_capacity_increase_config", self.base))
    }

    #[doc = "Get a reference to the value of field `eks_resource_scaling_config` after provisioning.\n"]
    pub fn eks_resource_scaling_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_resource_scaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_approval_config` after provisioning.\n"]
    pub fn execution_approval_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_approval_config", self.base))
    }

    #[doc = "Get a reference to the value of field `global_aurora_config` after provisioning.\n"]
    pub fn global_aurora_config(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_aurora_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_event_source_mapping_config` after provisioning.\n"]
    pub fn lambda_event_source_mapping_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_event_source_mapping_config", self.base))
    }

    #[doc = "Get a reference to the value of field `neptune_global_database_config` after provisioning.\n"]
    pub fn neptune_global_database_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.neptune_global_database_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_create_cross_region_read_replica_config` after provisioning.\n"]
    pub fn rds_create_cross_region_read_replica_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.rds_create_cross_region_read_replica_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rds_promote_read_replica_config` after provisioning.\n"]
    pub fn rds_promote_read_replica_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_promote_read_replica_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_switchover_read_replica_config` after provisioning.\n"]
    pub fn rds_switchover_read_replica_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_switchover_read_replica_config", self.base))
    }

    #[doc = "Get a reference to the value of field `region_switch_plan_config` after provisioning.\n"]
    pub fn region_switch_plan_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_switch_plan_config", self.base))
    }

    #[doc = "Get a reference to the value of field `route53_health_check_config` after provisioning.\n"]
    pub fn route53_health_check_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route53_health_check_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_Step_ParallelConfigDynamic {
    step: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step_ParallelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step>>,
    dynamic: ArcregionswitchPlan_Workflow_Step_ParallelConfigDynamic,
}

impl ArcregionswitchPlan_Workflow_Step_ParallelConfig {
    #[doc = "Set the field `step`.\n"]
    pub fn set_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ParallelConfig_Step>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step_ParallelConfig {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ParallelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step_ParallelConfig {}

impl BuildArcregionswitchPlan_Workflow_Step_ParallelConfig {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step_ParallelConfig {
        ArcregionswitchPlan_Workflow_Step_ParallelConfig {
            step: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_Step_ParallelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_Step_ParallelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArcregionswitchPlan_Workflow_Step_ParallelConfigRef {
        ArcregionswitchPlan_Workflow_Step_ParallelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_Step_ParallelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_ParallelConfig_StepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_Workflow_StepDynamic {
    arc_routing_control_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>>,
    aurora_provisioned_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>>,
    aurora_serverless_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>>,
    custom_action_lambda_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>>,
    document_db_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>>,
    ec2_asg_capacity_increase_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>>,
    ecs_capacity_increase_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>>,
    eks_resource_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>>,
    execution_approval_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>>,
    global_aurora_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>>,
    lambda_event_source_mapping_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>>,
    neptune_global_database_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>>,
    parallel_config: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_ParallelConfig>>,
    rds_create_cross_region_read_replica_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>>,
    rds_promote_read_replica_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>>,
    rds_switchover_read_replica_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>>,
    region_switch_plan_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>>,
    route53_health_check_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow_Step {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_block_type: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arc_routing_control_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora_provisioned_scaling_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora_serverless_scaling_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_action_lambda_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_db_config: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_asg_capacity_increase_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_capacity_increase_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_resource_scaling_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_approval_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_aurora_config: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_event_source_mapping_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    neptune_global_database_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallel_config: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_ParallelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_create_cross_region_read_replica_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_promote_read_replica_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_switchover_read_replica_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_switch_plan_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route53_health_check_config: std
    ::option
    ::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>>,
    dynamic: ArcregionswitchPlan_Workflow_StepDynamic,
}

impl ArcregionswitchPlan_Workflow_Step {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arc_routing_control_config`.\n"]
    pub fn set_arc_routing_control_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.arc_routing_control_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.arc_routing_control_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aurora_provisioned_scaling_config`.\n"]
    pub fn set_aurora_provisioned_scaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aurora_provisioned_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora_provisioned_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aurora_serverless_scaling_config`.\n"]
    pub fn set_aurora_serverless_scaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aurora_serverless_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora_serverless_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_action_lambda_config`.\n"]
    pub fn set_custom_action_lambda_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_action_lambda_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_action_lambda_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_db_config`.\n"]
    pub fn set_document_db_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_DocumentDbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.document_db_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.document_db_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_asg_capacity_increase_config`.\n"]
    pub fn set_ec2_asg_capacity_increase_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_asg_capacity_increase_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_asg_capacity_increase_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_capacity_increase_config`.\n"]
    pub fn set_ecs_capacity_increase_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecs_capacity_increase_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecs_capacity_increase_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eks_resource_scaling_config`.\n"]
    pub fn set_eks_resource_scaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eks_resource_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eks_resource_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_approval_config`.\n"]
    pub fn set_execution_approval_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_approval_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_approval_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `global_aurora_config`.\n"]
    pub fn set_global_aurora_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.global_aurora_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.global_aurora_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_event_source_mapping_config`.\n"]
    pub fn set_lambda_event_source_mapping_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_event_source_mapping_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_event_source_mapping_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `neptune_global_database_config`.\n"]
    pub fn set_neptune_global_database_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.neptune_global_database_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.neptune_global_database_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallel_config`.\n"]
    pub fn set_parallel_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_ParallelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parallel_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parallel_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_create_cross_region_read_replica_config`.\n"]
    pub fn set_rds_create_cross_region_read_replica_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_create_cross_region_read_replica_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_create_cross_region_read_replica_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_promote_read_replica_config`.\n"]
    pub fn set_rds_promote_read_replica_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_promote_read_replica_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_promote_read_replica_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_switchover_read_replica_config`.\n"]
    pub fn set_rds_switchover_read_replica_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_switchover_read_replica_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_switchover_read_replica_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region_switch_plan_config`.\n"]
    pub fn set_region_switch_plan_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region_switch_plan_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region_switch_plan_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route53_health_check_config`.\n"]
    pub fn set_route53_health_check_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route53_health_check_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route53_health_check_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow_Step {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow_Step {
    #[doc = ""]
    pub execution_block_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow_Step {
    pub fn build(self) -> ArcregionswitchPlan_Workflow_Step {
        ArcregionswitchPlan_Workflow_Step {
            description: core::default::Default::default(),
            execution_block_type: self.execution_block_type,
            name: self.name,
            arc_routing_control_config: core::default::Default::default(),
            aurora_provisioned_scaling_config: core::default::Default::default(),
            aurora_serverless_scaling_config: core::default::Default::default(),
            custom_action_lambda_config: core::default::Default::default(),
            document_db_config: core::default::Default::default(),
            ec2_asg_capacity_increase_config: core::default::Default::default(),
            ecs_capacity_increase_config: core::default::Default::default(),
            eks_resource_scaling_config: core::default::Default::default(),
            execution_approval_config: core::default::Default::default(),
            global_aurora_config: core::default::Default::default(),
            lambda_event_source_mapping_config: core::default::Default::default(),
            neptune_global_database_config: core::default::Default::default(),
            parallel_config: core::default::Default::default(),
            rds_create_cross_region_read_replica_config: core::default::Default::default(),
            rds_promote_read_replica_config: core::default::Default::default(),
            rds_switchover_read_replica_config: core::default::Default::default(),
            region_switch_plan_config: core::default::Default::default(),
            route53_health_check_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_Workflow_StepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_Workflow_StepRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_Workflow_StepRef {
        ArcregionswitchPlan_Workflow_StepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_Workflow_StepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_block_type` after provisioning.\n"]
    pub fn execution_block_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_block_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `arc_routing_control_config` after provisioning.\n"]
    pub fn arc_routing_control_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_ArcRoutingControlConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arc_routing_control_config", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora_provisioned_scaling_config` after provisioning.\n"]
    pub fn aurora_provisioned_scaling_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_AuroraProvisionedScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora_provisioned_scaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora_serverless_scaling_config` after provisioning.\n"]
    pub fn aurora_serverless_scaling_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_AuroraServerlessScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora_serverless_scaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_action_lambda_config` after provisioning.\n"]
    pub fn custom_action_lambda_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_CustomActionLambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_action_lambda_config", self.base))
    }

    #[doc = "Get a reference to the value of field `document_db_config` after provisioning.\n"]
    pub fn document_db_config(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_DocumentDbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_db_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ec2_asg_capacity_increase_config` after provisioning.\n"]
    pub fn ec2_asg_capacity_increase_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_Ec2AsgCapacityIncreaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_asg_capacity_increase_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ecs_capacity_increase_config` after provisioning.\n"]
    pub fn ecs_capacity_increase_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EcsCapacityIncreaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_capacity_increase_config", self.base))
    }

    #[doc = "Get a reference to the value of field `eks_resource_scaling_config` after provisioning.\n"]
    pub fn eks_resource_scaling_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_EksResourceScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_resource_scaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_approval_config` after provisioning.\n"]
    pub fn execution_approval_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_ExecutionApprovalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_approval_config", self.base))
    }

    #[doc = "Get a reference to the value of field `global_aurora_config` after provisioning.\n"]
    pub fn global_aurora_config(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_GlobalAuroraConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_aurora_config", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_event_source_mapping_config` after provisioning.\n"]
    pub fn lambda_event_source_mapping_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_LambdaEventSourceMappingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_event_source_mapping_config", self.base))
    }

    #[doc = "Get a reference to the value of field `neptune_global_database_config` after provisioning.\n"]
    pub fn neptune_global_database_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_NeptuneGlobalDatabaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.neptune_global_database_config", self.base))
    }

    #[doc = "Get a reference to the value of field `parallel_config` after provisioning.\n"]
    pub fn parallel_config(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_ParallelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallel_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_create_cross_region_read_replica_config` after provisioning.\n"]
    pub fn rds_create_cross_region_read_replica_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RdsCreateCrossRegionReadReplicaConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.rds_create_cross_region_read_replica_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rds_promote_read_replica_config` after provisioning.\n"]
    pub fn rds_promote_read_replica_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RdsPromoteReadReplicaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_promote_read_replica_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_switchover_read_replica_config` after provisioning.\n"]
    pub fn rds_switchover_read_replica_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RdsSwitchoverReadReplicaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_switchover_read_replica_config", self.base))
    }

    #[doc = "Get a reference to the value of field `region_switch_plan_config` after provisioning.\n"]
    pub fn region_switch_plan_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_RegionSwitchPlanConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_switch_plan_config", self.base))
    }

    #[doc = "Get a reference to the value of field `route53_health_check_config` after provisioning.\n"]
    pub fn route53_health_check_config(
        &self,
    ) -> terrars::ListRef<ArcregionswitchPlan_Workflow_Step_Route53HealthCheckConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route53_health_check_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ArcregionswitchPlan_WorkflowDynamic {
    step: std::option::Option<terrars::DynamicBlock<ArcregionswitchPlan_Workflow_Step>>,
}

#[derive(Serialize)]
pub struct ArcregionswitchPlan_Workflow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_description: std::option::Option<terrars::PrimField<std::string::String>>,
    workflow_target_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_target_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step: std::option::Option<std::vec::Vec<ArcregionswitchPlan_Workflow_Step>>,
    dynamic: ArcregionswitchPlan_WorkflowDynamic,
}

impl ArcregionswitchPlan_Workflow {
    #[doc = "Set the field `workflow_description`.\n"]
    pub fn set_workflow_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workflow_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workflow_target_region`.\n"]
    pub fn set_workflow_target_region(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workflow_target_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step`.\n"]
    pub fn set_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArcregionswitchPlan_Workflow_Step>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ArcregionswitchPlan_Workflow {
    type O = terrars::BlockAssignable<ArcregionswitchPlan_Workflow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArcregionswitchPlan_Workflow {
    #[doc = ""]
    pub workflow_target_action: terrars::PrimField<std::string::String>,
}

impl BuildArcregionswitchPlan_Workflow {
    pub fn build(self) -> ArcregionswitchPlan_Workflow {
        ArcregionswitchPlan_Workflow {
            workflow_description: core::default::Default::default(),
            workflow_target_action: self.workflow_target_action,
            workflow_target_region: core::default::Default::default(),
            step: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArcregionswitchPlan_WorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArcregionswitchPlan_WorkflowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArcregionswitchPlan_WorkflowRef {
        ArcregionswitchPlan_WorkflowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArcregionswitchPlan_WorkflowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workflow_description` after provisioning.\n"]
    pub fn workflow_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_description", self.base))
    }

    #[doc = "Get a reference to the value of field `workflow_target_action` after provisioning.\n"]
    pub fn workflow_target_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_target_action", self.base))
    }

    #[doc = "Get a reference to the value of field `workflow_target_region` after provisioning.\n"]
    pub fn workflow_target_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_target_region", self.base))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<ArcregionswitchPlan_Workflow_StepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step", self.base))
    }
}
