#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ArczonalshiftZonalAutoshiftConfigurationData_ {
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
    allowed_windows: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocked_dates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocked_windows: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_arn: terrars::PrimField<std::string::String>,
    zonal_autoshift_status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocking_alarms: std::option::Option<std::vec::Vec<ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outcome_alarms: std::option::Option<std::vec::Vec<ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms>>,
    dynamic: ArczonalshiftZonalAutoshiftConfigurationDynamic,
}

struct ArczonalshiftZonalAutoshiftConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ArczonalshiftZonalAutoshiftConfigurationData_>,
}

#[derive(Clone)]
pub struct ArczonalshiftZonalAutoshiftConfiguration(std::rc::Rc<ArczonalshiftZonalAutoshiftConfiguration_>);

impl ArczonalshiftZonalAutoshiftConfiguration {
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

    #[doc =
        "Set the field `allowed_windows`.\nList of time windows during which practice runs are allowed, in the format `Day:HH:MM-Day:HH:MM` (e.g., `Mon:09:00-Mon:17:00`). Cannot be used together with `blocked_windows`."]
    pub fn set_allowed_windows(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_windows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `blocked_dates`.\nList of dates when practice runs should not be started, in the format `YYYY-MM-DD`."]
    pub fn set_blocked_dates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().blocked_dates = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `blocked_windows`.\nList of time windows during which practice runs should not be started, in the format `Day:HH:MM-Day:HH:MM` (e.g., `Mon:00:00-Mon:08:00`). Cannot be used together with `allowed_windows`."]
    pub fn set_blocked_windows(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().blocked_windows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `blocking_alarms`.\n"]
    pub fn set_blocking_alarms(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blocking_alarms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.blocking_alarms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outcome_alarms`.\n"]
    pub fn set_outcome_alarms(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outcome_alarms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outcome_alarms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `allowed_windows` after provisioning.\nList of time windows during which practice runs are allowed, in the format `Day:HH:MM-Day:HH:MM` (e.g., `Mon:09:00-Mon:17:00`). Cannot be used together with `blocked_windows`."]
    pub fn allowed_windows(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_windows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocked_dates` after provisioning.\nList of dates when practice runs should not be started, in the format `YYYY-MM-DD`."]
    pub fn blocked_dates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocked_dates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocked_windows` after provisioning.\nList of time windows during which practice runs should not be started, in the format `Day:HH:MM-Day:HH:MM` (e.g., `Mon:00:00-Mon:08:00`). Cannot be used together with `allowed_windows`."]
    pub fn blocked_windows(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocked_windows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_arn` after provisioning.\nThe ARN of the managed resource to configure zonal autoshift for (e.g., an Application Load Balancer). Changing this creates a new resource."]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zonal_autoshift_status` after provisioning.\nThe status of zonal autoshift. Valid values: `ENABLED`, `DISABLED`."]
    pub fn zonal_autoshift_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zonal_autoshift_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocking_alarms` after provisioning.\n"]
    pub fn blocking_alarms(&self) -> terrars::ListRef<ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_alarms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outcome_alarms` after provisioning.\n"]
    pub fn outcome_alarms(&self) -> terrars::ListRef<ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outcome_alarms", self.extract_ref()))
    }
}

impl terrars::Referable for ArczonalshiftZonalAutoshiftConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ArczonalshiftZonalAutoshiftConfiguration { }

impl terrars::ToListMappable for ArczonalshiftZonalAutoshiftConfiguration {
    type O = terrars::ListRef<ArczonalshiftZonalAutoshiftConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ArczonalshiftZonalAutoshiftConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_arczonalshift_zonal_autoshift_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildArczonalshiftZonalAutoshiftConfiguration {
    pub tf_id: std::string::String,
    #[doc =
        "The ARN of the managed resource to configure zonal autoshift for (e.g., an Application Load Balancer). Changing this creates a new resource."]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = "The status of zonal autoshift. Valid values: `ENABLED`, `DISABLED`."]
    pub zonal_autoshift_status: terrars::PrimField<std::string::String>,
}

impl BuildArczonalshiftZonalAutoshiftConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> ArczonalshiftZonalAutoshiftConfiguration {
        let out =
            ArczonalshiftZonalAutoshiftConfiguration(std::rc::Rc::new(ArczonalshiftZonalAutoshiftConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(ArczonalshiftZonalAutoshiftConfigurationData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    allowed_windows: core::default::Default::default(),
                    blocked_dates: core::default::Default::default(),
                    blocked_windows: core::default::Default::default(),
                    region: core::default::Default::default(),
                    resource_arn: self.resource_arn,
                    zonal_autoshift_status: self.zonal_autoshift_status,
                    blocking_alarms: core::default::Default::default(),
                    outcome_alarms: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ArczonalshiftZonalAutoshiftConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArczonalshiftZonalAutoshiftConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ArczonalshiftZonalAutoshiftConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_windows` after provisioning.\nList of time windows during which practice runs are allowed, in the format `Day:HH:MM-Day:HH:MM` (e.g., `Mon:09:00-Mon:17:00`). Cannot be used together with `blocked_windows`."]
    pub fn allowed_windows(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_windows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocked_dates` after provisioning.\nList of dates when practice runs should not be started, in the format `YYYY-MM-DD`."]
    pub fn blocked_dates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocked_dates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocked_windows` after provisioning.\nList of time windows during which practice runs should not be started, in the format `Day:HH:MM-Day:HH:MM` (e.g., `Mon:00:00-Mon:08:00`). Cannot be used together with `allowed_windows`."]
    pub fn blocked_windows(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocked_windows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_arn` after provisioning.\nThe ARN of the managed resource to configure zonal autoshift for (e.g., an Application Load Balancer). Changing this creates a new resource."]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zonal_autoshift_status` after provisioning.\nThe status of zonal autoshift. Valid values: `ENABLED`, `DISABLED`."]
    pub fn zonal_autoshift_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zonal_autoshift_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocking_alarms` after provisioning.\n"]
    pub fn blocking_alarms(&self) -> terrars::ListRef<ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_alarms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outcome_alarms` after provisioning.\n"]
    pub fn outcome_alarms(&self) -> terrars::ListRef<ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outcome_alarms", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ArczonalshiftZonalAutoshiftConfigurationDynamic {
    blocking_alarms: std
    ::option
    ::Option<terrars::DynamicBlock<ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms>>,
    outcome_alarms: std
    ::option
    ::Option<terrars::DynamicBlock<ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms>>,
}

#[derive(Serialize)]
pub struct ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms {
    alarm_identifier: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms { }

impl terrars::ToListMappable for ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms {
    type O = terrars::BlockAssignable<ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms {
    #[doc = "ARN of the CloudWatch alarm."]
    pub alarm_identifier: terrars::PrimField<std::string::String>,
    #[doc = "Type of control condition. Valid value: `CLOUDWATCH`."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms {
    pub fn build(self) -> ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms {
        ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarms {
            alarm_identifier: self.alarm_identifier,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef {
        ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArczonalshiftZonalAutoshiftConfiguration_BlockingAlarmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alarm_identifier` after provisioning.\nARN of the CloudWatch alarm."]
    pub fn alarm_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_identifier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of control condition. Valid value: `CLOUDWATCH`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms {
    alarm_identifier: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms { }

impl terrars::ToListMappable for ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms {
    type O = terrars::BlockAssignable<ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms {
    #[doc = "ARN of the CloudWatch alarm."]
    pub alarm_identifier: terrars::PrimField<std::string::String>,
    #[doc = "Type of control condition. Valid value: `CLOUDWATCH`."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms {
    pub fn build(self) -> ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms {
        ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarms {
            alarm_identifier: self.alarm_identifier,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef {
        ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArczonalshiftZonalAutoshiftConfiguration_OutcomeAlarmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alarm_identifier` after provisioning.\nARN of the CloudWatch alarm."]
    pub fn alarm_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_identifier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of control condition. Valid value: `CLOUDWATCH`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
