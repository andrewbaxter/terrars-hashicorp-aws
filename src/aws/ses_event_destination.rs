use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SesEventDestinationData {
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
    configuration_set_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    matching_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_destination: std::option::Option<std::vec::Vec<SesEventDestinationCloudwatchDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_destination: std::option::Option<std::vec::Vec<SesEventDestinationKinesisDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_destination: std::option::Option<std::vec::Vec<SesEventDestinationSnsDestinationEl>>,
    dynamic: SesEventDestinationDynamic,
}

struct SesEventDestination_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SesEventDestinationData>,
}

#[derive(Clone)]
pub struct SesEventDestination(std::rc::Rc<SesEventDestination_>);

impl SesEventDestination {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_destination`.\n"]
    pub fn set_cloudwatch_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesEventDestinationCloudwatchDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_destination`.\n"]
    pub fn set_kinesis_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesEventDestinationKinesisDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns_destination`.\n"]
    pub fn set_sns_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesEventDestinationSnsDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `matching_types` after provisioning.\n"]
    pub fn matching_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.matching_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_destination` after provisioning.\n"]
    pub fn kinesis_destination(&self) -> terrars::ListRef<SesEventDestinationKinesisDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> terrars::ListRef<SesEventDestinationSnsDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_destination", self.extract_ref()))
    }
}

impl terrars::Referable for SesEventDestination {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SesEventDestination { }

impl terrars::ToListMappable for SesEventDestination {
    type O = terrars::ListRef<SesEventDestinationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SesEventDestination_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ses_event_destination".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesEventDestination {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub configuration_set_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub matching_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSesEventDestination {
    pub fn build(self, stack: &mut terrars::Stack) -> SesEventDestination {
        let out = SesEventDestination(std::rc::Rc::new(SesEventDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SesEventDestinationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                configuration_set_name: self.configuration_set_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                matching_types: self.matching_types,
                name: self.name,
                cloudwatch_destination: core::default::Default::default(),
                kinesis_destination: core::default::Default::default(),
                sns_destination: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SesEventDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesEventDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SesEventDestinationRef {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `matching_types` after provisioning.\n"]
    pub fn matching_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.matching_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_destination` after provisioning.\n"]
    pub fn kinesis_destination(&self) -> terrars::ListRef<SesEventDestinationKinesisDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(&self) -> terrars::ListRef<SesEventDestinationSnsDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_destination", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SesEventDestinationCloudwatchDestinationEl {
    default_value: terrars::PrimField<std::string::String>,
    dimension_name: terrars::PrimField<std::string::String>,
    value_source: terrars::PrimField<std::string::String>,
}

impl SesEventDestinationCloudwatchDestinationEl { }

impl terrars::ToListMappable for SesEventDestinationCloudwatchDestinationEl {
    type O = terrars::BlockAssignable<SesEventDestinationCloudwatchDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesEventDestinationCloudwatchDestinationEl {
    #[doc = ""]
    pub default_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub dimension_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_source: terrars::PrimField<std::string::String>,
}

impl BuildSesEventDestinationCloudwatchDestinationEl {
    pub fn build(self) -> SesEventDestinationCloudwatchDestinationEl {
        SesEventDestinationCloudwatchDestinationEl {
            default_value: self.default_value,
            dimension_name: self.dimension_name,
            value_source: self.value_source,
        }
    }
}

pub struct SesEventDestinationCloudwatchDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesEventDestinationCloudwatchDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesEventDestinationCloudwatchDestinationElRef {
        SesEventDestinationCloudwatchDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesEventDestinationCloudwatchDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_source` after provisioning.\n"]
    pub fn value_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SesEventDestinationKinesisDestinationEl {
    role_arn: terrars::PrimField<std::string::String>,
    stream_arn: terrars::PrimField<std::string::String>,
}

impl SesEventDestinationKinesisDestinationEl { }

impl terrars::ToListMappable for SesEventDestinationKinesisDestinationEl {
    type O = terrars::BlockAssignable<SesEventDestinationKinesisDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesEventDestinationKinesisDestinationEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildSesEventDestinationKinesisDestinationEl {
    pub fn build(self) -> SesEventDestinationKinesisDestinationEl {
        SesEventDestinationKinesisDestinationEl {
            role_arn: self.role_arn,
            stream_arn: self.stream_arn,
        }
    }
}

pub struct SesEventDestinationKinesisDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesEventDestinationKinesisDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesEventDestinationKinesisDestinationElRef {
        SesEventDestinationKinesisDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesEventDestinationKinesisDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesEventDestinationSnsDestinationEl {
    topic_arn: terrars::PrimField<std::string::String>,
}

impl SesEventDestinationSnsDestinationEl { }

impl terrars::ToListMappable for SesEventDestinationSnsDestinationEl {
    type O = terrars::BlockAssignable<SesEventDestinationSnsDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesEventDestinationSnsDestinationEl {
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildSesEventDestinationSnsDestinationEl {
    pub fn build(self) -> SesEventDestinationSnsDestinationEl {
        SesEventDestinationSnsDestinationEl { topic_arn: self.topic_arn }
    }
}

pub struct SesEventDestinationSnsDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesEventDestinationSnsDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesEventDestinationSnsDestinationElRef {
        SesEventDestinationSnsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesEventDestinationSnsDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SesEventDestinationDynamic {
    cloudwatch_destination: std::option::Option<terrars::DynamicBlock<SesEventDestinationCloudwatchDestinationEl>>,
    kinesis_destination: std::option::Option<terrars::DynamicBlock<SesEventDestinationKinesisDestinationEl>>,
    sns_destination: std::option::Option<terrars::DynamicBlock<SesEventDestinationSnsDestinationEl>>,
}
