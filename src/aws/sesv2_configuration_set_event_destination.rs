use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2ConfigurationSetEventDestinationData {
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
    event_destination_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_destination: std::option::Option<std::vec::Vec<Sesv2ConfigurationSetEventDestinationEventDestinationEl>>,
    dynamic: Sesv2ConfigurationSetEventDestinationDynamic,
}

struct Sesv2ConfigurationSetEventDestination_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Sesv2ConfigurationSetEventDestinationData>,
}

#[derive(Clone)]
pub struct Sesv2ConfigurationSetEventDestination(std::rc::Rc<Sesv2ConfigurationSetEventDestination_>);

impl Sesv2ConfigurationSetEventDestination {
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

    #[doc = "Set the field `event_destination`.\n"]
    pub fn set_event_destination(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination_name` after provisioning.\n"]
    pub fn event_destination_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_destination_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination` after provisioning.\n"]
    pub fn event_destination(&self) -> terrars::ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_destination", self.extract_ref()))
    }
}

impl terrars::Referable for Sesv2ConfigurationSetEventDestination {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Sesv2ConfigurationSetEventDestination { }

impl terrars::ToListMappable for Sesv2ConfigurationSetEventDestination {
    type O = terrars::ListRef<Sesv2ConfigurationSetEventDestinationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Sesv2ConfigurationSetEventDestination_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sesv2_configuration_set_event_destination".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2ConfigurationSetEventDestination {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub configuration_set_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub event_destination_name: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSetEventDestination {
    pub fn build(self, stack: &mut terrars::Stack) -> Sesv2ConfigurationSetEventDestination {
        let out = Sesv2ConfigurationSetEventDestination(std::rc::Rc::new(Sesv2ConfigurationSetEventDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Sesv2ConfigurationSetEventDestinationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                configuration_set_name: self.configuration_set_name,
                event_destination_name: self.event_destination_name,
                id: core::default::Default::default(),
                event_destination: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2ConfigurationSetEventDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetEventDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination_name` after provisioning.\n"]
    pub fn event_destination_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_destination_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_destination` after provisioning.\n"]
    pub fn event_destination(&self) -> terrars::ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_destination", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    default_dimension_value: terrars::PrimField<std::string::String>,
    dimension_name: terrars::PrimField<std::string::String>,
    dimension_value_source: terrars::PrimField<std::string::String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl { }

impl terrars
::ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    #[doc = ""]
    pub default_dimension_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub dimension_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub dimension_value_source: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
    pub fn build(
        self,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl {
            default_dimension_value: self.default_dimension_value,
            dimension_name: self.dimension_name,
            dimension_value_source: self.dimension_value_source,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_dimension_value` after provisioning.\n"]
    pub fn default_dimension_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_dimension_value", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_value_source` after provisioning.\n"]
    pub fn dimension_value_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_value_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDynamic {
    dimension_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl>,
    >,
    dynamic: Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDynamic,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    #[doc = "Set the field `dimension_configuration`.\n"]
    pub fn set_dimension_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    type O =
        terrars::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl {
            dimension_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension_configuration` after provisioning.\n"]
    pub fn dimension_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElDimensionConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    delivery_stream_arn: terrars::PrimField<std::string::String>,
    iam_role_arn: terrars::PrimField<std::string::String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl { }

impl terrars::ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    type O =
        terrars
        ::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    #[doc = ""]
    pub delivery_stream_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl {
            delivery_stream_arn: self.delivery_stream_arn,
            iam_role_arn: self.iam_role_arn,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream_arn` after provisioning.\n"]
    pub fn delivery_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    application_arn: terrars::PrimField<std::string::String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl { }

impl terrars::ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    #[doc = ""]
    pub application_arn: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl {
            application_arn: self.application_arn,
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    topic_arn: terrars::PrimField<std::string::String>,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl { }

impl terrars::ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl { topic_arn: self.topic_arn }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetEventDestinationEventDestinationElDynamic {
    cloud_watch_destination: std
    ::option
    ::Option<terrars::DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl>>,
    kinesis_firehose_destination: std
    ::option
    ::Option<
        terrars::DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl>,
    >,
    pinpoint_destination: std
    ::option
    ::Option<terrars::DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>>,
    sns_destination: std
    ::option
    ::Option<terrars::DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>>,
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetEventDestinationEventDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    matching_event_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_destination: std
    ::option
    ::Option<std::vec::Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_destination: std
    ::option
    ::Option<std::vec::Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinpoint_destination: std
    ::option
    ::Option<std::vec::Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_destination: std
    ::option
    ::Option<std::vec::Vec<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>>,
    dynamic: Sesv2ConfigurationSetEventDestinationEventDestinationElDynamic,
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_destination`.\n"]
    pub fn set_cloud_watch_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_watch_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_destination`.\n"]
    pub fn set_kinesis_firehose_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_firehose_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pinpoint_destination`.\n"]
    pub fn set_pinpoint_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pinpoint_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pinpoint_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns_destination`.\n"]
    pub fn set_sns_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sns_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Sesv2ConfigurationSetEventDestinationEventDestinationEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetEventDestinationEventDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetEventDestinationEventDestinationEl {
    #[doc = ""]
    pub matching_event_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSesv2ConfigurationSetEventDestinationEventDestinationEl {
    pub fn build(self) -> Sesv2ConfigurationSetEventDestinationEventDestinationEl {
        Sesv2ConfigurationSetEventDestinationEventDestinationEl {
            enabled: core::default::Default::default(),
            matching_event_types: self.matching_event_types,
            cloud_watch_destination: core::default::Default::default(),
            kinesis_firehose_destination: core::default::Default::default(),
            pinpoint_destination: core::default::Default::default(),
            sns_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
        Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetEventDestinationEventDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `matching_event_types` after provisioning.\n"]
    pub fn matching_event_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matching_event_types", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_destination` after provisioning.\n"]
    pub fn cloud_watch_destination(
        &self,
    ) -> terrars::ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElCloudWatchDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_watch_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_destination` after provisioning.\n"]
    pub fn kinesis_firehose_destination(
        &self,
    ) -> terrars::ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElKinesisFirehoseDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `pinpoint_destination` after provisioning.\n"]
    pub fn pinpoint_destination(
        &self,
    ) -> terrars::ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElPinpointDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pinpoint_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_destination` after provisioning.\n"]
    pub fn sns_destination(
        &self,
    ) -> terrars::ListRef<Sesv2ConfigurationSetEventDestinationEventDestinationElSnsDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetEventDestinationDynamic {
    event_destination: std
    ::option
    ::Option<terrars::DynamicBlock<Sesv2ConfigurationSetEventDestinationEventDestinationEl>>,
}
