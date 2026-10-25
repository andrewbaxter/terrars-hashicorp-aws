#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TimestreamqueryScheduledQueryData_ {
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
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    query_string: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_report_configuration: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_ErrorReportConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_run_summary: std::option::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_configuration: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_NotificationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recently_failed_runs: std::option::Option<std::vec::Vec<TimestreamqueryScheduledQuery_RecentlyFailedRuns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_configuration: std::option::Option<std::vec::Vec<TimestreamqueryScheduledQuery_ScheduleConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_configuration: std::option::Option<std::vec::Vec<TimestreamqueryScheduledQuery_TargetConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<TimestreamqueryScheduledQuery_Timeouts>,
    dynamic: TimestreamqueryScheduledQueryDynamic,
}

struct TimestreamqueryScheduledQuery_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TimestreamqueryScheduledQueryData_>,
}

#[derive(Clone)]
pub struct TimestreamqueryScheduledQuery(std::rc::Rc<TimestreamqueryScheduledQuery_>);

impl TimestreamqueryScheduledQuery {
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

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `error_report_configuration`.\n"]
    pub fn set_error_report_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_ErrorReportConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_report_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_report_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_run_summary`.\n"]
    pub fn set_last_run_summary(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().last_run_summary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.last_run_summary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_configuration`.\n"]
    pub fn set_notification_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_NotificationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recently_failed_runs`.\n"]
    pub fn set_recently_failed_runs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_RecentlyFailedRuns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recently_failed_runs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recently_failed_runs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule_configuration`.\n"]
    pub fn set_schedule_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_ScheduleConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_configuration`.\n"]
    pub fn set_target_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_TargetConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<TimestreamqueryScheduledQuery_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `next_invocation_time` after provisioning.\n"]
    pub fn next_invocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_invocation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `previous_invocation_time` after provisioning.\n"]
    pub fn previous_invocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.previous_invocation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_report_configuration` after provisioning.\n"]
    pub fn error_report_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_ErrorReportConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_report_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_run_summary` after provisioning.\n"]
    pub fn last_run_summary(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_run_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_NotificationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recently_failed_runs` after provisioning.\n"]
    pub fn recently_failed_runs(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_RecentlyFailedRunsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recently_failed_runs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_configuration` after provisioning.\n"]
    pub fn schedule_configuration(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_ScheduleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_TargetConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreamqueryScheduledQuery_TimeoutsRef {
        TimestreamqueryScheduledQuery_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for TimestreamqueryScheduledQuery {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TimestreamqueryScheduledQuery { }

impl terrars::ToListMappable for TimestreamqueryScheduledQuery {
    type O = terrars::ListRef<TimestreamqueryScheduledQueryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TimestreamqueryScheduledQuery_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_timestreamquery_scheduled_query".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTimestreamqueryScheduledQuery {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub query_string: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery {
    pub fn build(self, stack: &mut terrars::Stack) -> TimestreamqueryScheduledQuery {
        let out = TimestreamqueryScheduledQuery(std::rc::Rc::new(TimestreamqueryScheduledQuery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TimestreamqueryScheduledQueryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                kms_key_id: core::default::Default::default(),
                name: self.name,
                query_string: self.query_string,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                error_report_configuration: core::default::Default::default(),
                last_run_summary: core::default::Default::default(),
                notification_configuration: core::default::Default::default(),
                recently_failed_runs: core::default::Default::default(),
                schedule_configuration: core::default::Default::default(),
                target_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TimestreamqueryScheduledQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TimestreamqueryScheduledQueryRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `next_invocation_time` after provisioning.\n"]
    pub fn next_invocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_invocation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `previous_invocation_time` after provisioning.\n"]
    pub fn previous_invocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.previous_invocation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_report_configuration` after provisioning.\n"]
    pub fn error_report_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_ErrorReportConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_report_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_run_summary` after provisioning.\n"]
    pub fn last_run_summary(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_run_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_NotificationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recently_failed_runs` after provisioning.\n"]
    pub fn recently_failed_runs(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_RecentlyFailedRunsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recently_failed_runs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_configuration` after provisioning.\n"]
    pub fn schedule_configuration(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_ScheduleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_TargetConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreamqueryScheduledQuery_TimeoutsRef {
        TimestreamqueryScheduledQuery_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQueryDynamic {
    error_report_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_ErrorReportConfiguration>>,
    last_run_summary: std::option::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary>>,
    notification_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_NotificationConfiguration>>,
    recently_failed_runs: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_RecentlyFailedRuns>>,
    schedule_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_ScheduleConfiguration>>,
    target_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_TargetConfiguration>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
        TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration {
            bucket_name: self.bucket_name,
            encryption_option: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3ConfigurationRef {
        TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_ErrorReportConfigurationDynamic {
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_ErrorReportConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration>>,
    dynamic: TimestreamqueryScheduledQuery_ErrorReportConfigurationDynamic,
}

impl TimestreamqueryScheduledQuery_ErrorReportConfiguration {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3Configuration>,
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

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_ErrorReportConfiguration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_ErrorReportConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_ErrorReportConfiguration {}

impl BuildTimestreamqueryScheduledQuery_ErrorReportConfiguration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_ErrorReportConfiguration {
        TimestreamqueryScheduledQuery_ErrorReportConfiguration {
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_ErrorReportConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_ErrorReportConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_ErrorReportConfigurationRef {
        TimestreamqueryScheduledQuery_ErrorReportConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_ErrorReportConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_ErrorReportConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation {}

impl TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation { }

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation {
    type O =
        terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation {
        TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation {}
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocationRef {
        TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key` after provisioning.\n"]
    pub fn object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationDynamic {
    s3_report_location: std
    ::option
    ::Option<
        terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_report_location: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation>>,
    dynamic: TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationDynamic,
}

impl TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {
    #[doc = "Set the field `s3_report_location`.\n"]
    pub fn set_s3_report_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_report_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_report_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {
        TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation {
            s3_report_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef {
        TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_report_location` after provisioning.\n"]
    pub fn s3_report_location(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation_S3ReportLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_report_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats {}

impl TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats { }

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats {
        TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats {}
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef {
        TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes_metered` after provisioning.\n"]
    pub fn bytes_metered(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes_metered", self.base))
    }

    #[doc = "Get a reference to the value of field `cumulative_bytes_scanned` after provisioning.\n"]
    pub fn cumulative_bytes_scanned(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cumulative_bytes_scanned", self.base))
    }

    #[doc = "Get a reference to the value of field `data_writes` after provisioning.\n"]
    pub fn data_writes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_writes", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_time_in_millis` after provisioning.\n"]
    pub fn execution_time_in_millis(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_time_in_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `query_result_rows` after provisioning.\n"]
    pub fn query_result_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_result_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `records_ingested` after provisioning.\n"]
    pub fn records_ingested(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.records_ingested", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max {}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max { }

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max {
    type O =
        terrars
        ::BlockAssignable<
            TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max {}
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_MaxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_MaxRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_MaxRef {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_MaxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_MaxRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageDynamic {
    max: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std
    ::option
    ::Option<
        std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max>,
    >,
    dynamic: TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageDynamic,
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_Max,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {
    type O =
        terrars
        ::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage {
            max: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageRef {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(
        &self,
    ) -> terrars
    ::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage_MaxRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max {}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max { }

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max {
    type O =
        terrars
        ::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max {}
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_MaxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_MaxRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_MaxRef {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_MaxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_MaxRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeDynamic {
    max: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std
    ::option
    ::Option<
        std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max>,
    >,
    dynamic: TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeDynamic,
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_Max,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {
    type O =
        terrars
        ::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange {
            max: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeRef {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(
        &self,
    ) -> terrars
    ::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange_MaxRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseDynamic {
    query_spatial_coverage: std
    ::option
    ::Option<
        terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage>,
    >,
    query_temporal_range: std
    ::option
    ::Option<
        terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_spatial_coverage: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_temporal_range: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange>>,
    dynamic: TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseDynamic,
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {
    #[doc = "Set the field `query_spatial_coverage`.\n"]
    pub fn set_query_spatial_coverage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_spatial_coverage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_spatial_coverage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_temporal_range`.\n"]
    pub fn set_query_temporal_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRange,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_temporal_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_temporal_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse {
            query_spatial_coverage: core::default::Default::default(),
            query_temporal_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef {
        TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_bytes` after provisioning.\n"]
    pub fn output_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `output_rows` after provisioning.\n"]
    pub fn output_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `query_table_count` after provisioning.\n"]
    pub fn query_table_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_table_count", self.base))
    }

    #[doc = "Get a reference to the value of field `query_spatial_coverage` after provisioning.\n"]
    pub fn query_spatial_coverage(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QuerySpatialCoverageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_spatial_coverage", self.base))
    }

    #[doc = "Get a reference to the value of field `query_temporal_range` after provisioning.\n"]
    pub fn query_temporal_range(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse_QueryTemporalRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_temporal_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_LastRunSummaryDynamic {
    error_report_location: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>>,
    execution_stats: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>>,
    query_insights_response: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_LastRunSummary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_report_location: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_stats: std::option::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_insights_response: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>>,
    dynamic: TimestreamqueryScheduledQuery_LastRunSummaryDynamic,
}

impl TimestreamqueryScheduledQuery_LastRunSummary {
    #[doc = "Set the field `error_report_location`.\n"]
    pub fn set_error_report_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_report_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_report_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_stats`.\n"]
    pub fn set_execution_stats(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_stats = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_stats = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_insights_response`.\n"]
    pub fn set_query_insights_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_insights_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_insights_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_LastRunSummary {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_LastRunSummary {}

impl BuildTimestreamqueryScheduledQuery_LastRunSummary {
    pub fn build(self) -> TimestreamqueryScheduledQuery_LastRunSummary {
        TimestreamqueryScheduledQuery_LastRunSummary {
            error_report_location: core::default::Default::default(),
            execution_stats: core::default::Default::default(),
            query_insights_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_LastRunSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_LastRunSummaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_LastRunSummaryRef {
        TimestreamqueryScheduledQuery_LastRunSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_LastRunSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_time` after provisioning.\n"]
    pub fn invocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `run_status` after provisioning.\n"]
    pub fn run_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_status", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_time` after provisioning.\n"]
    pub fn trigger_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_time", self.base))
    }

    #[doc = "Get a reference to the value of field `error_report_location` after provisioning.\n"]
    pub fn error_report_location(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_report_location", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_stats` after provisioning.\n"]
    pub fn execution_stats(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_stats", self.base))
    }

    #[doc = "Get a reference to the value of field `query_insights_response` after provisioning.\n"]
    pub fn query_insights_response(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_response", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration {
    topic_arn: terrars::PrimField<std::string::String>,
}

impl TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration { }

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration {
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration {
        TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration { topic_arn: self.topic_arn }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfigurationRef {
        TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_NotificationConfigurationDynamic {
    sns_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_NotificationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_configuration: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration>>,
    dynamic: TimestreamqueryScheduledQuery_NotificationConfigurationDynamic,
}

impl TimestreamqueryScheduledQuery_NotificationConfiguration {
    #[doc = "Set the field `sns_configuration`.\n"]
    pub fn set_sns_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sns_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_NotificationConfiguration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_NotificationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_NotificationConfiguration {}

impl BuildTimestreamqueryScheduledQuery_NotificationConfiguration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_NotificationConfiguration {
        TimestreamqueryScheduledQuery_NotificationConfiguration {
            sns_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_NotificationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_NotificationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_NotificationConfigurationRef {
        TimestreamqueryScheduledQuery_NotificationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_NotificationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_configuration` after provisioning.\n"]
    pub fn sns_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_NotificationConfiguration_SnsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_RecentlyFailedRunsDynamic {
    error_report_location: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>>,
    execution_stats: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>>,
    query_insights_response: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_RecentlyFailedRuns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_report_location: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_stats: std::option::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_insights_response: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>>,
    dynamic: TimestreamqueryScheduledQuery_RecentlyFailedRunsDynamic,
}

impl TimestreamqueryScheduledQuery_RecentlyFailedRuns {
    #[doc = "Set the field `error_report_location`.\n"]
    pub fn set_error_report_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_report_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_report_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_stats`.\n"]
    pub fn set_execution_stats(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStats>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_stats = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_stats = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_insights_response`.\n"]
    pub fn set_query_insights_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponse>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_insights_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_insights_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_RecentlyFailedRuns {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_RecentlyFailedRuns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_RecentlyFailedRuns {}

impl BuildTimestreamqueryScheduledQuery_RecentlyFailedRuns {
    pub fn build(self) -> TimestreamqueryScheduledQuery_RecentlyFailedRuns {
        TimestreamqueryScheduledQuery_RecentlyFailedRuns {
            error_report_location: core::default::Default::default(),
            execution_stats: core::default::Default::default(),
            query_insights_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_RecentlyFailedRunsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_RecentlyFailedRunsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_RecentlyFailedRunsRef {
        TimestreamqueryScheduledQuery_RecentlyFailedRunsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_RecentlyFailedRunsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_time` after provisioning.\n"]
    pub fn invocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `run_status` after provisioning.\n"]
    pub fn run_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_status", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_time` after provisioning.\n"]
    pub fn trigger_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_time", self.base))
    }

    #[doc = "Get a reference to the value of field `error_report_location` after provisioning.\n"]
    pub fn error_report_location(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_ErrorReportLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_report_location", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_stats` after provisioning.\n"]
    pub fn execution_stats(&self) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_ExecutionStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_stats", self.base))
    }

    #[doc = "Get a reference to the value of field `query_insights_response` after provisioning.\n"]
    pub fn query_insights_response(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_LastRunSummary_QueryInsightsResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_response", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_ScheduleConfiguration {
    schedule_expression: terrars::PrimField<std::string::String>,
}

impl TimestreamqueryScheduledQuery_ScheduleConfiguration { }

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_ScheduleConfiguration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_ScheduleConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_ScheduleConfiguration {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_ScheduleConfiguration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_ScheduleConfiguration {
        TimestreamqueryScheduledQuery_ScheduleConfiguration { schedule_expression: self.schedule_expression }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_ScheduleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_ScheduleConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_ScheduleConfigurationRef {
        TimestreamqueryScheduledQuery_ScheduleConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_ScheduleConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping {
    dimension_value_type: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping { }

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping {
    type O =
        terrars
        ::BlockAssignable<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping {
    #[doc = ""]
    pub dimension_value_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping {
    pub fn build(self) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping {
            dimension_value_type: self.dimension_value_type,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMappingRef {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension_value_type` after provisioning.\n"]
    pub fn dimension_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
    measure_value_type: terrars::PrimField<std::string::String>,
    source_column: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_multi_measure_attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
    #[doc = "Set the field `target_multi_measure_attribute_name`.\n"]
    pub fn set_target_multi_measure_attribute_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_multi_measure_attribute_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
    type O =
        terrars
        ::BlockAssignable<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
    #[doc = ""]
    pub measure_value_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_column: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping {
            measure_value_type: self.measure_value_type,
            source_column: self.source_column,
            target_multi_measure_attribute_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `measure_value_type` after provisioning.\n"]
    pub fn measure_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.measure_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_column` after provisioning.\n"]
    pub fn source_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_column", self.base))
    }

    #[doc = "Get a reference to the value of field `target_multi_measure_attribute_name` after provisioning.\n"]
    pub fn target_multi_measure_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_multi_measure_attribute_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingDynamic {
    multi_measure_attribute_mapping: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    measure_name: std::option::Option<terrars::PrimField<std::string::String>>,
    measure_value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_measure_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_measure_attribute_mapping: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
        >,
    >,
    dynamic: TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingDynamic,
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
    #[doc = "Set the field `measure_name`.\n"]
    pub fn set_measure_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.measure_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_column`.\n"]
    pub fn set_source_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_measure_name`.\n"]
    pub fn set_target_measure_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_measure_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_measure_attribute_mapping`.\n"]
    pub fn set_multi_measure_attribute_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_measure_attribute_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_measure_attribute_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
    type O =
        terrars
        ::BlockAssignable<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
    #[doc = ""]
    pub measure_value_type: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping {
            measure_name: core::default::Default::default(),
            measure_value_type: self.measure_value_type,
            source_column: core::default::Default::default(),
            target_measure_name: core::default::Default::default(),
            multi_measure_attribute_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingRef {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `measure_name` after provisioning.\n"]
    pub fn measure_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.measure_name", self.base))
    }

    #[doc = "Get a reference to the value of field `measure_value_type` after provisioning.\n"]
    pub fn measure_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.measure_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_column` after provisioning.\n"]
    pub fn source_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_column", self.base))
    }

    #[doc = "Get a reference to the value of field `target_measure_name` after provisioning.\n"]
    pub fn target_measure_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_measure_name", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_measure_attribute_mapping` after provisioning.\n"]
    pub fn multi_measure_attribute_mapping(
        &self,
    ) -> terrars
    ::ListRef<
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_measure_attribute_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsDynamic {
    multi_measure_attribute_mapping: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_multi_measure_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_measure_attribute_mapping: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
        >,
    >,
    dynamic: TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsDynamic,
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {
    #[doc = "Set the field `target_multi_measure_name`.\n"]
    pub fn set_target_multi_measure_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_multi_measure_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_measure_attribute_mapping`.\n"]
    pub fn set_multi_measure_attribute_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_measure_attribute_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_measure_attribute_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {
    type O =
        terrars
        ::BlockAssignable<
            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {}

impl BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {
    pub fn build(
        self,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings {
            target_multi_measure_name: core::default::Default::default(),
            multi_measure_attribute_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsRef {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_multi_measure_name` after provisioning.\n"]
    pub fn target_multi_measure_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_multi_measure_name", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_measure_attribute_mapping` after provisioning.\n"]
    pub fn multi_measure_attribute_mapping(
        &self,
    ) -> terrars
    ::ListRef<
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping_MultiMeasureAttributeMappingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_measure_attribute_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationDynamic {
    dimension_mapping: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping>,
    >,
    mixed_measure_mapping: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping>,
    >,
    multi_measure_mappings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    measure_name_column: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    time_column: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_mapping: std
    ::option
    ::Option<
        std::vec::Vec<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mixed_measure_mapping: std
    ::option
    ::Option<
        std::vec::Vec<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_measure_mappings: std
    ::option
    ::Option<
        std::vec::Vec<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings>,
    >,
    dynamic: TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationDynamic,
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
    #[doc = "Set the field `measure_name_column`.\n"]
    pub fn set_measure_name_column(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.measure_name_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension_mapping`.\n"]
    pub fn set_dimension_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mixed_measure_mapping`.\n"]
    pub fn set_mixed_measure_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mixed_measure_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mixed_measure_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_measure_mappings`.\n"]
    pub fn set_multi_measure_mappings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_measure_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_measure_mappings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_column: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration {
            database_name: self.database_name,
            measure_name_column: core::default::Default::default(),
            table_name: self.table_name,
            time_column: self.time_column,
            dimension_mapping: core::default::Default::default(),
            mixed_measure_mapping: core::default::Default::default(),
            multi_measure_mappings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationRef {
        TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `measure_name_column` after provisioning.\n"]
    pub fn measure_name_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.measure_name_column", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `time_column` after provisioning.\n"]
    pub fn time_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_column", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_mapping` after provisioning.\n"]
    pub fn dimension_mapping(
        &self,
    ) -> terrars
    ::ListRef<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_DimensionMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `mixed_measure_mapping` after provisioning.\n"]
    pub fn mixed_measure_mapping(
        &self,
    ) -> terrars
    ::ListRef<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MixedMeasureMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_measure_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_measure_mappings` after provisioning.\n"]
    pub fn multi_measure_mappings(
        &self,
    ) -> terrars
    ::ListRef<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration_MultiMeasureMappingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_measure_mappings", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamqueryScheduledQuery_TargetConfigurationDynamic {
    timestream_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration>>,
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_TargetConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestream_configuration: std
    ::option
    ::Option<std::vec::Vec<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration>>,
    dynamic: TimestreamqueryScheduledQuery_TargetConfigurationDynamic,
}

impl TimestreamqueryScheduledQuery_TargetConfiguration {
    #[doc = "Set the field `timestream_configuration`.\n"]
    pub fn set_timestream_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timestream_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timestream_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_TargetConfiguration {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_TargetConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_TargetConfiguration {}

impl BuildTimestreamqueryScheduledQuery_TargetConfiguration {
    pub fn build(self) -> TimestreamqueryScheduledQuery_TargetConfiguration {
        TimestreamqueryScheduledQuery_TargetConfiguration {
            timestream_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TargetConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_TargetConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamqueryScheduledQuery_TargetConfigurationRef {
        TimestreamqueryScheduledQuery_TargetConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TargetConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timestream_configuration` after provisioning.\n"]
    pub fn timestream_configuration(
        &self,
    ) -> terrars::ListRef<TimestreamqueryScheduledQuery_TargetConfiguration_TimestreamConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestream_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamqueryScheduledQuery_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TimestreamqueryScheduledQuery_Timeouts {
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

impl terrars::ToListMappable for TimestreamqueryScheduledQuery_Timeouts {
    type O = terrars::BlockAssignable<TimestreamqueryScheduledQuery_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamqueryScheduledQuery_Timeouts {}

impl BuildTimestreamqueryScheduledQuery_Timeouts {
    pub fn build(self) -> TimestreamqueryScheduledQuery_Timeouts {
        TimestreamqueryScheduledQuery_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamqueryScheduledQuery_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamqueryScheduledQuery_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TimestreamqueryScheduledQuery_TimeoutsRef {
        TimestreamqueryScheduledQuery_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamqueryScheduledQuery_TimeoutsRef {
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
