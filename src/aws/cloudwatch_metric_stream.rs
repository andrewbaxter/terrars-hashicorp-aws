use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchMetricStreamData {
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
    firehose_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    output_format: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_filter: std::option::Option<std::vec::Vec<CloudwatchMetricStreamExcludeFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_filter: std::option::Option<std::vec::Vec<CloudwatchMetricStreamIncludeFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistics_configuration: std::option::Option<std::vec::Vec<CloudwatchMetricStreamStatisticsConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudwatchMetricStreamTimeoutsEl>,
    dynamic: CloudwatchMetricStreamDynamic,
}

struct CloudwatchMetricStream_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchMetricStreamData>,
}

#[derive(Clone)]
pub struct CloudwatchMetricStream(std::rc::Rc<CloudwatchMetricStream_>);

impl CloudwatchMetricStream {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `exclude_filter`.\n"]
    pub fn set_exclude_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricStreamExcludeFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_filter`.\n"]
    pub fn set_include_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricStreamIncludeFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statistics_configuration`.\n"]
    pub fn set_statistics_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().statistics_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.statistics_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudwatchMetricStreamTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firehose_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_date` after provisioning.\n"]
    pub fn last_update_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudwatchMetricStreamTimeoutsElRef {
        CloudwatchMetricStreamTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchMetricStream {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchMetricStream { }

impl terrars::ToListMappable for CloudwatchMetricStream {
    type O = terrars::ListRef<CloudwatchMetricStreamRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchMetricStream_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatch_metric_stream".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchMetricStream {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firehose_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub output_format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricStream {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchMetricStream {
        let out = CloudwatchMetricStream(std::rc::Rc::new(CloudwatchMetricStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchMetricStreamData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                firehose_arn: self.firehose_arn,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                output_format: self.output_format,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_filter: core::default::Default::default(),
                include_filter: core::default::Default::default(),
                statistics_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchMetricStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricStreamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchMetricStreamRef {
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

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firehose_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_date` after provisioning.\n"]
    pub fn last_update_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudwatchMetricStreamTimeoutsElRef {
        CloudwatchMetricStreamTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricStreamExcludeFilterEl {
    namespace: terrars::PrimField<std::string::String>,
}

impl CloudwatchMetricStreamExcludeFilterEl { }

impl terrars::ToListMappable for CloudwatchMetricStreamExcludeFilterEl {
    type O = terrars::BlockAssignable<CloudwatchMetricStreamExcludeFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricStreamExcludeFilterEl {
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricStreamExcludeFilterEl {
    pub fn build(self) -> CloudwatchMetricStreamExcludeFilterEl {
        CloudwatchMetricStreamExcludeFilterEl { namespace: self.namespace }
    }
}

pub struct CloudwatchMetricStreamExcludeFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricStreamExcludeFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricStreamExcludeFilterElRef {
        CloudwatchMetricStreamExcludeFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricStreamExcludeFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricStreamIncludeFilterEl {
    namespace: terrars::PrimField<std::string::String>,
}

impl CloudwatchMetricStreamIncludeFilterEl { }

impl terrars::ToListMappable for CloudwatchMetricStreamIncludeFilterEl {
    type O = terrars::BlockAssignable<CloudwatchMetricStreamIncludeFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricStreamIncludeFilterEl {
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricStreamIncludeFilterEl {
    pub fn build(self) -> CloudwatchMetricStreamIncludeFilterEl {
        CloudwatchMetricStreamIncludeFilterEl { namespace: self.namespace }
    }
}

pub struct CloudwatchMetricStreamIncludeFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricStreamIncludeFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricStreamIncludeFilterElRef {
        CloudwatchMetricStreamIncludeFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricStreamIncludeFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
}

impl CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl { }

impl terrars::ToListMappable for CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    type O = terrars::BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
    pub fn build(self) -> CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
        CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
        }
    }
}

pub struct CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
        CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchMetricStreamStatisticsConfigurationElDynamic {
    include_metric: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchMetricStreamStatisticsConfigurationEl {
    additional_statistics: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_metric: std::option::Option<std::vec::Vec<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>>,
    dynamic: CloudwatchMetricStreamStatisticsConfigurationElDynamic,
}

impl CloudwatchMetricStreamStatisticsConfigurationEl {
    #[doc = "Set the field `include_metric`.\n"]
    pub fn set_include_metric(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationElIncludeMetricEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_metric = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_metric = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchMetricStreamStatisticsConfigurationEl {
    type O = terrars::BlockAssignable<CloudwatchMetricStreamStatisticsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricStreamStatisticsConfigurationEl {
    #[doc = ""]
    pub additional_statistics: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchMetricStreamStatisticsConfigurationEl {
    pub fn build(self) -> CloudwatchMetricStreamStatisticsConfigurationEl {
        CloudwatchMetricStreamStatisticsConfigurationEl {
            additional_statistics: self.additional_statistics,
            include_metric: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchMetricStreamStatisticsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricStreamStatisticsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchMetricStreamStatisticsConfigurationElRef {
        CloudwatchMetricStreamStatisticsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricStreamStatisticsConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_statistics` after provisioning.\n"]
    pub fn additional_statistics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_statistics", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchMetricStreamTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchMetricStreamTimeoutsEl {
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

impl terrars::ToListMappable for CloudwatchMetricStreamTimeoutsEl {
    type O = terrars::BlockAssignable<CloudwatchMetricStreamTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchMetricStreamTimeoutsEl {}

impl BuildCloudwatchMetricStreamTimeoutsEl {
    pub fn build(self) -> CloudwatchMetricStreamTimeoutsEl {
        CloudwatchMetricStreamTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchMetricStreamTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchMetricStreamTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchMetricStreamTimeoutsElRef {
        CloudwatchMetricStreamTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchMetricStreamTimeoutsElRef {
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
struct CloudwatchMetricStreamDynamic {
    exclude_filter: std::option::Option<terrars::DynamicBlock<CloudwatchMetricStreamExcludeFilterEl>>,
    include_filter: std::option::Option<terrars::DynamicBlock<CloudwatchMetricStreamIncludeFilterEl>>,
    statistics_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchMetricStreamStatisticsConfigurationEl>>,
}
