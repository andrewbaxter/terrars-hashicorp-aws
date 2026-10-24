use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EvidentlyProjectData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_delivery: std::option::Option<std::vec::Vec<EvidentlyProjectDataDeliveryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EvidentlyProjectTimeoutsEl>,
    dynamic: EvidentlyProjectDynamic,
}

struct EvidentlyProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EvidentlyProjectData>,
}

#[derive(Clone)]
pub struct EvidentlyProject(std::rc::Rc<EvidentlyProject_>);

impl EvidentlyProject {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_delivery`.\n"]
    pub fn set_data_delivery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyProjectDataDeliveryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_delivery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_delivery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EvidentlyProjectTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `active_experiment_count` after provisioning.\n"]
    pub fn active_experiment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_launch_count` after provisioning.\n"]
    pub fn active_launch_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_count` after provisioning.\n"]
    pub fn experiment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_count` after provisioning.\n"]
    pub fn feature_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_count` after provisioning.\n"]
    pub fn launch_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_delivery` after provisioning.\n"]
    pub fn data_delivery(&self) -> terrars::ListRef<EvidentlyProjectDataDeliveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyProjectTimeoutsElRef {
        EvidentlyProjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EvidentlyProject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EvidentlyProject { }

impl terrars::ToListMappable for EvidentlyProject {
    type O = terrars::ListRef<EvidentlyProjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EvidentlyProject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_evidently_project".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEvidentlyProject {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyProject {
    pub fn build(self, stack: &mut terrars::Stack) -> EvidentlyProject {
        let out = EvidentlyProject(std::rc::Rc::new(EvidentlyProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EvidentlyProjectData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_delivery: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EvidentlyProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EvidentlyProjectRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_experiment_count` after provisioning.\n"]
    pub fn active_experiment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_launch_count` after provisioning.\n"]
    pub fn active_launch_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_count` after provisioning.\n"]
    pub fn experiment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.experiment_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_count` after provisioning.\n"]
    pub fn feature_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_count` after provisioning.\n"]
    pub fn launch_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_delivery` after provisioning.\n"]
    pub fn data_delivery(&self) -> terrars::ListRef<EvidentlyProjectDataDeliveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyProjectTimeoutsElRef {
        EvidentlyProjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    type O = terrars::BlockAssignable<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectDataDeliveryElCloudwatchLogsEl {}

impl BuildEvidentlyProjectDataDeliveryElCloudwatchLogsEl {
    pub fn build(self) -> EvidentlyProjectDataDeliveryElCloudwatchLogsEl {
        EvidentlyProjectDataDeliveryElCloudwatchLogsEl { log_group: core::default::Default::default() }
    }
}

pub struct EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
        EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectDataDeliveryElCloudwatchLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyProjectDataDeliveryElS3DestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyProjectDataDeliveryElS3DestinationEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyProjectDataDeliveryElS3DestinationEl {
    type O = terrars::BlockAssignable<EvidentlyProjectDataDeliveryElS3DestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectDataDeliveryElS3DestinationEl {}

impl BuildEvidentlyProjectDataDeliveryElS3DestinationEl {
    pub fn build(self) -> EvidentlyProjectDataDeliveryElS3DestinationEl {
        EvidentlyProjectDataDeliveryElS3DestinationEl {
            bucket: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyProjectDataDeliveryElS3DestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyProjectDataDeliveryElS3DestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EvidentlyProjectDataDeliveryElS3DestinationElRef {
        EvidentlyProjectDataDeliveryElS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectDataDeliveryElS3DestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyProjectDataDeliveryElDynamic {
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>>,
    s3_destination: std::option::Option<terrars::DynamicBlock<EvidentlyProjectDataDeliveryElS3DestinationEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyProjectDataDeliveryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_destination: std::option::Option<std::vec::Vec<EvidentlyProjectDataDeliveryElS3DestinationEl>>,
    dynamic: EvidentlyProjectDataDeliveryElDynamic,
}

impl EvidentlyProjectDataDeliveryEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyProjectDataDeliveryElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyProjectDataDeliveryElS3DestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EvidentlyProjectDataDeliveryEl {
    type O = terrars::BlockAssignable<EvidentlyProjectDataDeliveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectDataDeliveryEl {}

impl BuildEvidentlyProjectDataDeliveryEl {
    pub fn build(self) -> EvidentlyProjectDataDeliveryEl {
        EvidentlyProjectDataDeliveryEl {
            cloudwatch_logs: core::default::Default::default(),
            s3_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyProjectDataDeliveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyProjectDataDeliveryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyProjectDataDeliveryElRef {
        EvidentlyProjectDataDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectDataDeliveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<EvidentlyProjectDataDeliveryElCloudwatchLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<EvidentlyProjectDataDeliveryElS3DestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyProjectTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyProjectTimeoutsEl {
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

impl terrars::ToListMappable for EvidentlyProjectTimeoutsEl {
    type O = terrars::BlockAssignable<EvidentlyProjectTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyProjectTimeoutsEl {}

impl BuildEvidentlyProjectTimeoutsEl {
    pub fn build(self) -> EvidentlyProjectTimeoutsEl {
        EvidentlyProjectTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyProjectTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyProjectTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyProjectTimeoutsElRef {
        EvidentlyProjectTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyProjectTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct EvidentlyProjectDynamic {
    data_delivery: std::option::Option<terrars::DynamicBlock<EvidentlyProjectDataDeliveryEl>>,
}
