use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SyntheticsCanaryData {
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
    artifact_s3_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_lambda: std::option::Option<terrars::PrimField<bool>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_retention_period: std::option::Option<terrars::PrimField<f64>>,
    handler: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    runtime_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_canary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_config: std::option::Option<std::vec::Vec<SyntheticsCanaryArtifactConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_config: std::option::Option<std::vec::Vec<SyntheticsCanaryRunConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<SyntheticsCanaryScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SyntheticsCanaryVpcConfigEl>>,
    dynamic: SyntheticsCanaryDynamic,
}

struct SyntheticsCanary_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SyntheticsCanaryData>,
}

#[derive(Clone)]
pub struct SyntheticsCanary(std::rc::Rc<SyntheticsCanary_>);

impl SyntheticsCanary {
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

    #[doc = "Set the field `delete_lambda`.\n"]
    pub fn set_delete_lambda(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_lambda = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_retention_period`.\n"]
    pub fn set_failure_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().failure_retention_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key`.\n"]
    pub fn set_s3_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_version`.\n"]
    pub fn set_s3_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_canary`.\n"]
    pub fn set_start_canary(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_canary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_retention_period`.\n"]
    pub fn set_success_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().success_retention_period = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `zip_file`.\n"]
    pub fn set_zip_file(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zip_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `artifact_config`.\n"]
    pub fn set_artifact_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SyntheticsCanaryArtifactConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run_config`.\n"]
    pub fn set_run_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SyntheticsCanaryRunConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::BlockAssignable<SyntheticsCanaryScheduleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SyntheticsCanaryVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_s3_location` after provisioning.\n"]
    pub fn artifact_s3_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_s3_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_lambda` after provisioning.\n"]
    pub fn delete_lambda(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_lambda", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_arn` after provisioning.\n"]
    pub fn engine_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_retention_period` after provisioning.\n"]
    pub fn failure_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.handler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_version` after provisioning.\n"]
    pub fn s3_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_canary` after provisioning.\n"]
    pub fn start_canary(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_canary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_retention_period` after provisioning.\n"]
    pub fn success_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeline` after provisioning.\n"]
    pub fn timeline(&self) -> terrars::ListRef<SyntheticsCanaryTimelineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zip_file` after provisioning.\n"]
    pub fn zip_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_file", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_config` after provisioning.\n"]
    pub fn artifact_config(&self) -> terrars::ListRef<SyntheticsCanaryArtifactConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifact_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_config` after provisioning.\n"]
    pub fn run_config(&self) -> terrars::ListRef<SyntheticsCanaryRunConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<SyntheticsCanaryScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SyntheticsCanaryVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for SyntheticsCanary {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SyntheticsCanary { }

impl terrars::ToListMappable for SyntheticsCanary {
    type O = terrars::ListRef<SyntheticsCanaryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SyntheticsCanary_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_synthetics_canary".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSyntheticsCanary {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub artifact_s3_location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub handler: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub runtime_version: terrars::PrimField<std::string::String>,
}

impl BuildSyntheticsCanary {
    pub fn build(self, stack: &mut terrars::Stack) -> SyntheticsCanary {
        let out = SyntheticsCanary(std::rc::Rc::new(SyntheticsCanary_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SyntheticsCanaryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                artifact_s3_location: self.artifact_s3_location,
                delete_lambda: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                failure_retention_period: core::default::Default::default(),
                handler: self.handler,
                id: core::default::Default::default(),
                name: self.name,
                runtime_version: self.runtime_version,
                s3_bucket: core::default::Default::default(),
                s3_key: core::default::Default::default(),
                s3_version: core::default::Default::default(),
                start_canary: core::default::Default::default(),
                success_retention_period: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                zip_file: core::default::Default::default(),
                artifact_config: core::default::Default::default(),
                run_config: core::default::Default::default(),
                schedule: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SyntheticsCanaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SyntheticsCanaryRef {
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

    #[doc = "Get a reference to the value of field `artifact_s3_location` after provisioning.\n"]
    pub fn artifact_s3_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_s3_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_lambda` after provisioning.\n"]
    pub fn delete_lambda(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_lambda", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_arn` after provisioning.\n"]
    pub fn engine_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_retention_period` after provisioning.\n"]
    pub fn failure_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.handler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_version` after provisioning.\n"]
    pub fn s3_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_canary` after provisioning.\n"]
    pub fn start_canary(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_canary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_retention_period` after provisioning.\n"]
    pub fn success_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeline` after provisioning.\n"]
    pub fn timeline(&self) -> terrars::ListRef<SyntheticsCanaryTimelineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zip_file` after provisioning.\n"]
    pub fn zip_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_file", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_config` after provisioning.\n"]
    pub fn artifact_config(&self) -> terrars::ListRef<SyntheticsCanaryArtifactConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifact_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_config` after provisioning.\n"]
    pub fn run_config(&self) -> terrars::ListRef<SyntheticsCanaryRunConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<SyntheticsCanaryScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SyntheticsCanaryVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryTimelineEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_modified: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_started: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_stopped: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SyntheticsCanaryTimelineEl {
    #[doc = "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_modified`.\n"]
    pub fn set_last_modified(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_modified = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_started`.\n"]
    pub fn set_last_started(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_started = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_stopped`.\n"]
    pub fn set_last_stopped(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_stopped = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SyntheticsCanaryTimelineEl {
    type O = terrars::BlockAssignable<SyntheticsCanaryTimelineEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryTimelineEl {}

impl BuildSyntheticsCanaryTimelineEl {
    pub fn build(self) -> SyntheticsCanaryTimelineEl {
        SyntheticsCanaryTimelineEl {
            created: core::default::Default::default(),
            last_modified: core::default::Default::default(),
            last_started: core::default::Default::default(),
            last_stopped: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryTimelineElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryTimelineElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SyntheticsCanaryTimelineElRef {
        SyntheticsCanaryTimelineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryTimelineElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.base))
    }

    #[doc = "Get a reference to the value of field `last_started` after provisioning.\n"]
    pub fn last_started(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_started", self.base))
    }

    #[doc = "Get a reference to the value of field `last_stopped` after provisioning.\n"]
    pub fn last_stopped(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_stopped", self.base))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryArtifactConfigElS3EncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SyntheticsCanaryArtifactConfigElS3EncryptionEl {
    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SyntheticsCanaryArtifactConfigElS3EncryptionEl {
    type O = terrars::BlockAssignable<SyntheticsCanaryArtifactConfigElS3EncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryArtifactConfigElS3EncryptionEl {}

impl BuildSyntheticsCanaryArtifactConfigElS3EncryptionEl {
    pub fn build(self) -> SyntheticsCanaryArtifactConfigElS3EncryptionEl {
        SyntheticsCanaryArtifactConfigElS3EncryptionEl {
            encryption_mode: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
        SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryArtifactConfigElS3EncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SyntheticsCanaryArtifactConfigElDynamic {
    s3_encryption: std::option::Option<terrars::DynamicBlock<SyntheticsCanaryArtifactConfigElS3EncryptionEl>>,
}

#[derive(Serialize)]
pub struct SyntheticsCanaryArtifactConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_encryption: std::option::Option<std::vec::Vec<SyntheticsCanaryArtifactConfigElS3EncryptionEl>>,
    dynamic: SyntheticsCanaryArtifactConfigElDynamic,
}

impl SyntheticsCanaryArtifactConfigEl {
    #[doc = "Set the field `s3_encryption`.\n"]
    pub fn set_s3_encryption(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SyntheticsCanaryArtifactConfigElS3EncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SyntheticsCanaryArtifactConfigEl {
    type O = terrars::BlockAssignable<SyntheticsCanaryArtifactConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryArtifactConfigEl {}

impl BuildSyntheticsCanaryArtifactConfigEl {
    pub fn build(self) -> SyntheticsCanaryArtifactConfigEl {
        SyntheticsCanaryArtifactConfigEl {
            s3_encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryArtifactConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryArtifactConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SyntheticsCanaryArtifactConfigElRef {
        SyntheticsCanaryArtifactConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryArtifactConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_encryption` after provisioning.\n"]
    pub fn s3_encryption(&self) -> terrars::ListRef<SyntheticsCanaryArtifactConfigElS3EncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_encryption", self.base))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryRunConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_tracing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_in_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SyntheticsCanaryRunConfigEl {
    #[doc = "Set the field `active_tracing`.\n"]
    pub fn set_active_tracing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active_tracing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_in_mb`.\n"]
    pub fn set_memory_in_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_in_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_seconds`.\n"]
    pub fn set_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SyntheticsCanaryRunConfigEl {
    type O = terrars::BlockAssignable<SyntheticsCanaryRunConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryRunConfigEl {}

impl BuildSyntheticsCanaryRunConfigEl {
    pub fn build(self) -> SyntheticsCanaryRunConfigEl {
        SyntheticsCanaryRunConfigEl {
            active_tracing: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            memory_in_mb: core::default::Default::default(),
            timeout_in_seconds: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryRunConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryRunConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SyntheticsCanaryRunConfigElRef {
        SyntheticsCanaryRunConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryRunConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_tracing` after provisioning.\n"]
    pub fn active_tracing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_tracing", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_in_mb` after provisioning.\n"]
    pub fn memory_in_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    expression: terrars::PrimField<std::string::String>,
}

impl SyntheticsCanaryScheduleEl {
    #[doc = "Set the field `duration_in_seconds`.\n"]
    pub fn set_duration_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.duration_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SyntheticsCanaryScheduleEl {
    type O = terrars::BlockAssignable<SyntheticsCanaryScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryScheduleEl {
    #[doc = ""]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSyntheticsCanaryScheduleEl {
    pub fn build(self) -> SyntheticsCanaryScheduleEl {
        SyntheticsCanaryScheduleEl {
            duration_in_seconds: core::default::Default::default(),
            expression: self.expression,
        }
    }
}

pub struct SyntheticsCanaryScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SyntheticsCanaryScheduleElRef {
        SyntheticsCanaryScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration_in_seconds` after provisioning.\n"]
    pub fn duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct SyntheticsCanaryVpcConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SyntheticsCanaryVpcConfigEl {
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

impl terrars::ToListMappable for SyntheticsCanaryVpcConfigEl {
    type O = terrars::BlockAssignable<SyntheticsCanaryVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSyntheticsCanaryVpcConfigEl {}

impl BuildSyntheticsCanaryVpcConfigEl {
    pub fn build(self) -> SyntheticsCanaryVpcConfigEl {
        SyntheticsCanaryVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct SyntheticsCanaryVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SyntheticsCanaryVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SyntheticsCanaryVpcConfigElRef {
        SyntheticsCanaryVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SyntheticsCanaryVpcConfigElRef {
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SyntheticsCanaryDynamic {
    artifact_config: std::option::Option<terrars::DynamicBlock<SyntheticsCanaryArtifactConfigEl>>,
    run_config: std::option::Option<terrars::DynamicBlock<SyntheticsCanaryRunConfigEl>>,
    schedule: std::option::Option<terrars::DynamicBlock<SyntheticsCanaryScheduleEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<SyntheticsCanaryVpcConfigEl>>,
}
