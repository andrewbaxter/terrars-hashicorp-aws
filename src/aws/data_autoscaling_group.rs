use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAutoscalingGroupData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataAutoscalingGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAutoscalingGroupData>,
}

#[derive(Clone)]
pub struct DataAutoscalingGroup(Rc<DataAutoscalingGroup_>);

impl DataAutoscalingGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_cooldown", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_capacity_type` after provisioning.\n"]
    pub fn desired_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_metrics` after provisioning.\n"]
    pub fn enabled_metrics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_metrics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_grace_period` after provisioning.\n"]
    pub fn health_check_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_grace_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_type` after provisioning.\n"]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<DataAutoscalingGroupLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.load_balancers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_instances_protected_from_scale_in` after provisioning.\n"]
    pub fn new_instances_protected_from_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_instances_protected_from_scale_in", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_linked_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.termination_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_zone_identifier", self.extract_ref()))
    }
}

impl Datasource for DataAutoscalingGroup {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataAutoscalingGroup {
    type O = ListRef<DataAutoscalingGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAutoscalingGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_autoscaling_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAutoscalingGroup {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataAutoscalingGroup {
    pub fn build(self, stack: &mut Stack) -> DataAutoscalingGroup {
        let out = DataAutoscalingGroup(Rc::new(DataAutoscalingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAutoscalingGroupData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAutoscalingGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAutoscalingGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_cooldown` after provisioning.\n"]
    pub fn default_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_cooldown", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_capacity_type` after provisioning.\n"]
    pub fn desired_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_metrics` after provisioning.\n"]
    pub fn enabled_metrics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_metrics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_grace_period` after provisioning.\n"]
    pub fn health_check_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_grace_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `health_check_type` after provisioning.\n"]
    pub fn health_check_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `launch_configuration` after provisioning.\n"]
    pub fn launch_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<DataAutoscalingGroupLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.load_balancers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_instances_protected_from_scale_in` after provisioning.\n"]
    pub fn new_instances_protected_from_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_instances_protected_from_scale_in", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_linked_role_arn` after provisioning.\n"]
    pub fn service_linked_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_linked_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_group_arns` after provisioning.\n"]
    pub fn target_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_group_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `termination_policies` after provisioning.\n"]
    pub fn termination_policies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.termination_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_zone_identifier` after provisioning.\n"]
    pub fn vpc_zone_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_zone_identifier", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAutoscalingGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataAutoscalingGroupLaunchTemplateEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataAutoscalingGroupLaunchTemplateEl {
    type O = BlockAssignable<DataAutoscalingGroupLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAutoscalingGroupLaunchTemplateEl {}

impl BuildDataAutoscalingGroupLaunchTemplateEl {
    pub fn build(self) -> DataAutoscalingGroupLaunchTemplateEl {
        DataAutoscalingGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataAutoscalingGroupLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAutoscalingGroupLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataAutoscalingGroupLaunchTemplateElRef {
        DataAutoscalingGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAutoscalingGroupLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
