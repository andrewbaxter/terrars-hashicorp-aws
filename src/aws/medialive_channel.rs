#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MedialiveChannelData_ {
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
    channel_class: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_channel: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdi_input_specification: std::option::Option<std::vec::Vec<MedialiveChannel_CdiInputSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<MedialiveChannel_Destinations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoder_settings: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_attachments: std::option::Option<std::vec::Vec<MedialiveChannel_InputAttachments>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_specification: std::option::Option<std::vec::Vec<MedialiveChannel_InputSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance: std::option::Option<std::vec::Vec<MedialiveChannel_Maintenance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MedialiveChannel_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<MedialiveChannel_Vpc>>,
    dynamic: MedialiveChannelDynamic,
}

struct MedialiveChannel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MedialiveChannelData_>,
}

#[derive(Clone)]
pub struct MedialiveChannel(std::rc::Rc<MedialiveChannel_>);

impl MedialiveChannel {
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

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_channel`.\n"]
    pub fn set_start_channel(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_channel = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cdi_input_specification`.\n"]
    pub fn set_cdi_input_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_CdiInputSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cdi_input_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cdi_input_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encoder_settings`.\n"]
    pub fn set_encoder_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encoder_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encoder_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_attachments`.\n"]
    pub fn set_input_attachments(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_InputAttachments>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_attachments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_attachments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_specification`.\n"]
    pub fn set_input_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_InputSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance`.\n"]
    pub fn set_maintenance(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_Maintenance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MedialiveChannel_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_Vpc>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_class` after provisioning.\n"]
    pub fn channel_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_channel` after provisioning.\n"]
    pub fn start_channel(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdi_input_specification` after provisioning.\n"]
    pub fn cdi_input_specification(&self) -> terrars::ListRef<MedialiveChannel_CdiInputSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdi_input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoder_settings` after provisioning.\n"]
    pub fn encoder_settings(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encoder_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_specification` after provisioning.\n"]
    pub fn input_specification(&self) -> terrars::ListRef<MedialiveChannel_InputSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> terrars::ListRef<MedialiveChannel_MaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveChannel_TimeoutsRef {
        MedialiveChannel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MedialiveChannel_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

impl terrars::Referable for MedialiveChannel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MedialiveChannel { }

impl terrars::ToListMappable for MedialiveChannel {
    type O = terrars::ListRef<MedialiveChannelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MedialiveChannel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_medialive_channel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMedialiveChannel {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub channel_class: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel {
    pub fn build(self, stack: &mut terrars::Stack) -> MedialiveChannel {
        let out = MedialiveChannel(std::rc::Rc::new(MedialiveChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MedialiveChannelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                channel_class: self.channel_class,
                id: core::default::Default::default(),
                log_level: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                start_channel: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cdi_input_specification: core::default::Default::default(),
                destinations: core::default::Default::default(),
                encoder_settings: core::default::Default::default(),
                input_attachments: core::default::Default::default(),
                input_specification: core::default::Default::default(),
                maintenance: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MedialiveChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MedialiveChannelRef {
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

    #[doc = "Get a reference to the value of field `channel_class` after provisioning.\n"]
    pub fn channel_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_channel` after provisioning.\n"]
    pub fn start_channel(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdi_input_specification` after provisioning.\n"]
    pub fn cdi_input_specification(&self) -> terrars::ListRef<MedialiveChannel_CdiInputSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdi_input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoder_settings` after provisioning.\n"]
    pub fn encoder_settings(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encoder_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_specification` after provisioning.\n"]
    pub fn input_specification(&self) -> terrars::ListRef<MedialiveChannel_InputSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> terrars::ListRef<MedialiveChannel_MaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveChannel_TimeoutsRef {
        MedialiveChannel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MedialiveChannel_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelDynamic {
    cdi_input_specification: std::option::Option<terrars::DynamicBlock<MedialiveChannel_CdiInputSpecification>>,
    destinations: std::option::Option<terrars::DynamicBlock<MedialiveChannel_Destinations>>,
    encoder_settings: std::option::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings>>,
    input_attachments: std::option::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments>>,
    input_specification: std::option::Option<terrars::DynamicBlock<MedialiveChannel_InputSpecification>>,
    maintenance: std::option::Option<terrars::DynamicBlock<MedialiveChannel_Maintenance>>,
    vpc: std::option::Option<terrars::DynamicBlock<MedialiveChannel_Vpc>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_CdiInputSpecification {
    resolution: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_CdiInputSpecification { }

impl terrars::ToListMappable for MedialiveChannel_CdiInputSpecification {
    type O = terrars::BlockAssignable<MedialiveChannel_CdiInputSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_CdiInputSpecification {
    #[doc = ""]
    pub resolution: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_CdiInputSpecification {
    pub fn build(self) -> MedialiveChannel_CdiInputSpecification {
        MedialiveChannel_CdiInputSpecification { resolution: self.resolution }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_CdiInputSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_CdiInputSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_CdiInputSpecificationRef {
        MedialiveChannel_CdiInputSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_CdiInputSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_Destinations_MediaPackageSettings {
    channel_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_Destinations_MediaPackageSettings { }

impl terrars::ToListMappable for MedialiveChannel_Destinations_MediaPackageSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_Destinations_MediaPackageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Destinations_MediaPackageSettings {
    #[doc = ""]
    pub channel_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_Destinations_MediaPackageSettings {
    pub fn build(self) -> MedialiveChannel_Destinations_MediaPackageSettings {
        MedialiveChannel_Destinations_MediaPackageSettings { channel_id: self.channel_id }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_Destinations_MediaPackageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_Destinations_MediaPackageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_Destinations_MediaPackageSettingsRef {
        MedialiveChannel_Destinations_MediaPackageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_Destinations_MediaPackageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_Destinations_MultiplexSettings {
    multiplex_id: terrars::PrimField<std::string::String>,
    program_name: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_Destinations_MultiplexSettings { }

impl terrars::ToListMappable for MedialiveChannel_Destinations_MultiplexSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_Destinations_MultiplexSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Destinations_MultiplexSettings {
    #[doc = ""]
    pub multiplex_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub program_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_Destinations_MultiplexSettings {
    pub fn build(self) -> MedialiveChannel_Destinations_MultiplexSettings {
        MedialiveChannel_Destinations_MultiplexSettings {
            multiplex_id: self.multiplex_id,
            program_name: self.program_name,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_Destinations_MultiplexSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_Destinations_MultiplexSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_Destinations_MultiplexSettingsRef {
        MedialiveChannel_Destinations_MultiplexSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_Destinations_MultiplexSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `multiplex_id` after provisioning.\n"]
    pub fn multiplex_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiplex_id", self.base))
    }

    #[doc = "Get a reference to the value of field `program_name` after provisioning.\n"]
    pub fn program_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_name", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_Destinations_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_Destinations_Settings {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_name`.\n"]
    pub fn set_stream_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_Destinations_Settings {
    type O = terrars::BlockAssignable<MedialiveChannel_Destinations_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Destinations_Settings {}

impl BuildMedialiveChannel_Destinations_Settings {
    pub fn build(self) -> MedialiveChannel_Destinations_Settings {
        MedialiveChannel_Destinations_Settings {
            password_param: core::default::Default::default(),
            stream_name: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_Destinations_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_Destinations_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_Destinations_SettingsRef {
        MedialiveChannel_Destinations_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_Destinations_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_DestinationsDynamic {
    media_package_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_Destinations_MediaPackageSettings>>,
    multiplex_settings: std::option::Option<terrars::DynamicBlock<MedialiveChannel_Destinations_MultiplexSettings>>,
    settings: std::option::Option<terrars::DynamicBlock<MedialiveChannel_Destinations_Settings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_Destinations {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_package_settings: std::option::Option<std::vec::Vec<MedialiveChannel_Destinations_MediaPackageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_settings: std::option::Option<std::vec::Vec<MedialiveChannel_Destinations_MultiplexSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<MedialiveChannel_Destinations_Settings>>,
    dynamic: MedialiveChannel_DestinationsDynamic,
}

impl MedialiveChannel_Destinations {
    #[doc = "Set the field `media_package_settings`.\n"]
    pub fn set_media_package_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_Destinations_MediaPackageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.media_package_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.media_package_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multiplex_settings`.\n"]
    pub fn set_multiplex_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_Destinations_MultiplexSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multiplex_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multiplex_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_Destinations_Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_Destinations {
    type O = terrars::BlockAssignable<MedialiveChannel_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Destinations {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_Destinations {
    pub fn build(self) -> MedialiveChannel_Destinations {
        MedialiveChannel_Destinations {
            id: self.id,
            media_package_settings: core::default::Default::default(),
            multiplex_settings: core::default::Default::default(),
            settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_DestinationsRef {
        MedialiveChannel_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `multiplex_settings` after provisioning.\n"]
    pub fn multiplex_settings(&self) -> terrars::ListRef<MedialiveChannel_Destinations_MultiplexSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_lkfs: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `algorithm_control`.\n"]
    pub fn set_algorithm_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_lkfs`.\n"]
    pub fn set_target_lkfs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_lkfs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings {
            algorithm: core::default::Default::default(),
            algorithm_control: core::default::Default::default(),
            target_lkfs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `algorithm_control` after provisioning.\n"]
    pub fn algorithm_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_control", self.base))
    }

    #[doc = "Get a reference to the value of field `target_lkfs` after provisioning.\n"]
    pub fn target_lkfs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_lkfs", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {
    cbet_check_digit_string: terrars::PrimField<std::string::String>,
    cbet_stepaside: terrars::PrimField<std::string::String>,
    csid: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {
    #[doc = ""]
    pub cbet_check_digit_string: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub cbet_stepaside: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub csid: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings {
            cbet_check_digit_string: self.cbet_check_digit_string,
            cbet_stepaside: self.cbet_stepaside,
            csid: self.csid,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cbet_check_digit_string` after provisioning.\n"]
    pub fn cbet_check_digit_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cbet_check_digit_string", self.base))
    }

    #[doc = "Get a reference to the value of field `cbet_stepaside` after provisioning.\n"]
    pub fn cbet_stepaside(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cbet_stepaside", self.base))
    }

    #[doc = "Get a reference to the value of field `csid` after provisioning.\n"]
    pub fn csid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csid", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {
    check_digit_string: terrars::PrimField<std::string::String>,
    sid: terrars::PrimField<f64>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {
    #[doc = ""]
    pub check_digit_string: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sid: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings {
            check_digit_string: self.check_digit_string,
            sid: self.sid,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_digit_string` after provisioning.\n"]
    pub fn check_digit_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_digit_string", self.base))
    }

    #[doc = "Get a reference to the value of field `sid` after provisioning.\n"]
    pub fn sid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sid", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsDynamic {
    nielsen_cbet_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings,
        >,
    >,
    nielsen_naes_ii_nw_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_cbet_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_naes_ii_nw_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {
    #[doc = "Set the field `nielsen_distribution_type`.\n"]
    pub fn set_nielsen_distribution_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nielsen_distribution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nielsen_cbet_settings`.\n"]
    pub fn set_nielsen_cbet_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nielsen_cbet_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nielsen_cbet_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nielsen_naes_ii_nw_settings`.\n"]
    pub fn set_nielsen_naes_ii_nw_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nielsen_naes_ii_nw_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nielsen_naes_ii_nw_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings {
            nielsen_distribution_type: core::default::Default::default(),
            nielsen_cbet_settings: core::default::Default::default(),
            nielsen_naes_ii_nw_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nielsen_distribution_type` after provisioning.\n"]
    pub fn nielsen_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nielsen_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_cbet_settings` after provisioning.\n"]
    pub fn nielsen_cbet_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenCbetSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_cbet_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_naes_ii_nw_settings` after provisioning.\n"]
    pub fn nielsen_naes_ii_nw_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings_NielsenNaesIiNwSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_naes_ii_nw_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsDynamic {
    nielsen_watermarks_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_watermarks_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {
    #[doc = "Set the field `nielsen_watermarks_settings`.\n"]
    pub fn set_nielsen_watermarks_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nielsen_watermarks_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nielsen_watermarks_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings {
            nielsen_watermarks_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nielsen_watermarks_settings` after provisioning.\n"]
    pub fn nielsen_watermarks_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings_NielsenWatermarksSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_watermarks_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_control_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vbr_quality: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coding_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_type`.\n"]
    pub fn set_input_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_control_mode`.\n"]
    pub fn set_rate_control_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rate_control_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raw_format`.\n"]
    pub fn set_raw_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vbr_quality`.\n"]
    pub fn set_vbr_quality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vbr_quality = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            input_type: core::default::Default::default(),
            profile: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            raw_format: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
            spec: core::default::Default::default(),
            vbr_quality: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `input_type` after provisioning.\n"]
    pub fn input_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_type", self.base))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\n"]
    pub fn rate_control_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_control_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_format` after provisioning.\n"]
    pub fn raw_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_format", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spec", self.base))
    }

    #[doc = "Get a reference to the value of field `vbr_quality` after provisioning.\n"]
    pub fn vbr_quality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vbr_quality", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitstream_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialnorm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drc_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lfe_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_control: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitstream_mode`.\n"]
    pub fn set_bitstream_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bitstream_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coding_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialnorm`.\n"]
    pub fn set_dialnorm(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dialnorm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drc_profile`.\n"]
    pub fn set_drc_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.drc_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lfe_filter`.\n"]
    pub fn set_lfe_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lfe_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_control`.\n"]
    pub fn set_metadata_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata_control = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings {
            bitrate: core::default::Default::default(),
            bitstream_mode: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            dialnorm: core::default::Default::default(),
            drc_profile: core::default::Default::default(),
            lfe_filter: core::default::Default::default(),
            metadata_control: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3SettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `bitstream_mode` after provisioning.\n"]
    pub fn bitstream_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitstream_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `dialnorm` after provisioning.\n"]
    pub fn dialnorm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialnorm", self.base))
    }

    #[doc = "Get a reference to the value of field `drc_profile` after provisioning.\n"]
    pub fn drc_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drc_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `lfe_filter` after provisioning.\n"]
    pub fn lfe_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfe_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_control` after provisioning.\n"]
    pub fn metadata_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_control", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialnorm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drc_line: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drc_rf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    height_trim: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surround_trim: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coding_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialnorm`.\n"]
    pub fn set_dialnorm(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dialnorm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drc_line`.\n"]
    pub fn set_drc_line(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.drc_line = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drc_rf`.\n"]
    pub fn set_drc_rf(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.drc_rf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `height_trim`.\n"]
    pub fn set_height_trim(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.height_trim = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `surround_trim`.\n"]
    pub fn set_surround_trim(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.surround_trim = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {
    type O =
        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            dialnorm: core::default::Default::default(),
            drc_line: core::default::Default::default(),
            drc_rf: core::default::Default::default(),
            height_trim: core::default::Default::default(),
            surround_trim: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `dialnorm` after provisioning.\n"]
    pub fn dialnorm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialnorm", self.base))
    }

    #[doc = "Get a reference to the value of field `drc_line` after provisioning.\n"]
    pub fn drc_line(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drc_line", self.base))
    }

    #[doc = "Get a reference to the value of field `drc_rf` after provisioning.\n"]
    pub fn drc_rf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drc_rf", self.base))
    }

    #[doc = "Get a reference to the value of field `height_trim` after provisioning.\n"]
    pub fn height_trim(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height_trim", self.base))
    }

    #[doc = "Get a reference to the value of field `surround_trim` after provisioning.\n"]
    pub fn surround_trim(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.surround_trim", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attenuation_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitstream_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dc_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialnorm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drc_line: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drc_rf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lfe_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lfe_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lo_ro_center_mix_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lo_ro_surround_mix_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lt_rt_center_mix_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lt_rt_surround_mix_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passthrough_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stereo_downmix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surround_ex_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surround_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {
    #[doc = "Set the field `attenuation_control`.\n"]
    pub fn set_attenuation_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.attenuation_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitstream_mode`.\n"]
    pub fn set_bitstream_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bitstream_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coding_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dc_filter`.\n"]
    pub fn set_dc_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dc_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialnorm`.\n"]
    pub fn set_dialnorm(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dialnorm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drc_line`.\n"]
    pub fn set_drc_line(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.drc_line = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drc_rf`.\n"]
    pub fn set_drc_rf(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.drc_rf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lfe_control`.\n"]
    pub fn set_lfe_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lfe_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lfe_filter`.\n"]
    pub fn set_lfe_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lfe_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lo_ro_center_mix_level`.\n"]
    pub fn set_lo_ro_center_mix_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lo_ro_center_mix_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lo_ro_surround_mix_level`.\n"]
    pub fn set_lo_ro_surround_mix_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lo_ro_surround_mix_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lt_rt_center_mix_level`.\n"]
    pub fn set_lt_rt_center_mix_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lt_rt_center_mix_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lt_rt_surround_mix_level`.\n"]
    pub fn set_lt_rt_surround_mix_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lt_rt_surround_mix_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_control`.\n"]
    pub fn set_metadata_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `passthrough_control`.\n"]
    pub fn set_passthrough_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.passthrough_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phase_control`.\n"]
    pub fn set_phase_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stereo_downmix`.\n"]
    pub fn set_stereo_downmix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stereo_downmix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `surround_ex_mode`.\n"]
    pub fn set_surround_ex_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.surround_ex_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `surround_mode`.\n"]
    pub fn set_surround_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.surround_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings {
            attenuation_control: core::default::Default::default(),
            bitrate: core::default::Default::default(),
            bitstream_mode: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            dc_filter: core::default::Default::default(),
            dialnorm: core::default::Default::default(),
            drc_line: core::default::Default::default(),
            drc_rf: core::default::Default::default(),
            lfe_control: core::default::Default::default(),
            lfe_filter: core::default::Default::default(),
            lo_ro_center_mix_level: core::default::Default::default(),
            lo_ro_surround_mix_level: core::default::Default::default(),
            lt_rt_center_mix_level: core::default::Default::default(),
            lt_rt_surround_mix_level: core::default::Default::default(),
            metadata_control: core::default::Default::default(),
            passthrough_control: core::default::Default::default(),
            phase_control: core::default::Default::default(),
            stereo_downmix: core::default::Default::default(),
            surround_ex_mode: core::default::Default::default(),
            surround_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3SettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attenuation_control` after provisioning.\n"]
    pub fn attenuation_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attenuation_control", self.base))
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `bitstream_mode` after provisioning.\n"]
    pub fn bitstream_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitstream_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `dc_filter` after provisioning.\n"]
    pub fn dc_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dc_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `dialnorm` after provisioning.\n"]
    pub fn dialnorm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialnorm", self.base))
    }

    #[doc = "Get a reference to the value of field `drc_line` after provisioning.\n"]
    pub fn drc_line(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drc_line", self.base))
    }

    #[doc = "Get a reference to the value of field `drc_rf` after provisioning.\n"]
    pub fn drc_rf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drc_rf", self.base))
    }

    #[doc = "Get a reference to the value of field `lfe_control` after provisioning.\n"]
    pub fn lfe_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfe_control", self.base))
    }

    #[doc = "Get a reference to the value of field `lfe_filter` after provisioning.\n"]
    pub fn lfe_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfe_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `lo_ro_center_mix_level` after provisioning.\n"]
    pub fn lo_ro_center_mix_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lo_ro_center_mix_level", self.base))
    }

    #[doc = "Get a reference to the value of field `lo_ro_surround_mix_level` after provisioning.\n"]
    pub fn lo_ro_surround_mix_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lo_ro_surround_mix_level", self.base))
    }

    #[doc = "Get a reference to the value of field `lt_rt_center_mix_level` after provisioning.\n"]
    pub fn lt_rt_center_mix_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lt_rt_center_mix_level", self.base))
    }

    #[doc = "Get a reference to the value of field `lt_rt_surround_mix_level` after provisioning.\n"]
    pub fn lt_rt_surround_mix_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lt_rt_surround_mix_level", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_control` after provisioning.\n"]
    pub fn metadata_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_control", self.base))
    }

    #[doc = "Get a reference to the value of field `passthrough_control` after provisioning.\n"]
    pub fn passthrough_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passthrough_control", self.base))
    }

    #[doc = "Get a reference to the value of field `phase_control` after provisioning.\n"]
    pub fn phase_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase_control", self.base))
    }

    #[doc = "Get a reference to the value of field `stereo_downmix` after provisioning.\n"]
    pub fn stereo_downmix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stereo_downmix", self.base))
    }

    #[doc = "Get a reference to the value of field `surround_ex_mode` after provisioning.\n"]
    pub fn surround_ex_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.surround_ex_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `surround_mode` after provisioning.\n"]
    pub fn surround_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.surround_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {
    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coding_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2SettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings {}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings { }

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bit_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {
    #[doc = "Set the field `bit_depth`.\n"]
    pub fn set_bit_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bit_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coding_mode`.\n"]
    pub fn set_coding_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coding_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings {
            bit_depth: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bit_depth` after provisioning.\n"]
    pub fn bit_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bit_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `coding_mode` after provisioning.\n"]
    pub fn coding_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coding_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsDynamic {
    aac_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings>>,
    ac3_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings>>,
    eac3_atmos_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings>,
    >,
    eac3_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings>>,
    mp2_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings>>,
    pass_through_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings>,
    >,
    wav_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aac_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ac3_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eac3_atmos_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eac3_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mp2_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pass_through_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wav_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {
    #[doc = "Set the field `aac_settings`.\n"]
    pub fn set_aac_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aac_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aac_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ac3_settings`.\n"]
    pub fn set_ac3_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ac3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ac3_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eac3_atmos_settings`.\n"]
    pub fn set_eac3_atmos_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eac3_atmos_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eac3_atmos_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eac3_settings`.\n"]
    pub fn set_eac3_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eac3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eac3_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mp2_settings`.\n"]
    pub fn set_mp2_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mp2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mp2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pass_through_settings`.\n"]
    pub fn set_pass_through_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pass_through_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pass_through_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `wav_settings`.\n"]
    pub fn set_wav_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.wav_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.wav_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings {
            aac_settings: core::default::Default::default(),
            ac3_settings: core::default::Default::default(),
            eac3_atmos_settings: core::default::Default::default(),
            eac3_settings: core::default::Default::default(),
            mp2_settings: core::default::Default::default(),
            pass_through_settings: core::default::Default::default(),
            wav_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aac_settings` after provisioning.\n"]
    pub fn aac_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_AacSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aac_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ac3_settings` after provisioning.\n"]
    pub fn ac3_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Ac3SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ac3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `eac3_atmos_settings` after provisioning.\n"]
    pub fn eac3_atmos_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3AtmosSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eac3_atmos_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `eac3_settings` after provisioning.\n"]
    pub fn eac3_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Eac3SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eac3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `mp2_settings` after provisioning.\n"]
    pub fn mp2_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_Mp2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mp2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pass_through_settings` after provisioning.\n"]
    pub fn pass_through_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_PassThroughSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pass_through_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `wav_settings` after provisioning.\n"]
    pub fn wav_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings_WavSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wav_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels {
    gain: terrars::PrimField<f64>,
    input_channel: terrars::PrimField<f64>,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels {
    #[doc = ""]
    pub gain: terrars::PrimField<f64>,
    #[doc = ""]
    pub input_channel: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels {
        MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels {
            gain: self.gain,
            input_channel: self.input_channel,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevelsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gain` after provisioning.\n"]
    pub fn gain(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gain", self.base))
    }

    #[doc = "Get a reference to the value of field `input_channel` after provisioning.\n"]
    pub fn input_channel(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_channel", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsDynamic {
    input_channel_levels: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
    output_channel: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_channel_levels: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsDynamic,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
    #[doc = "Set the field `input_channel_levels`.\n"]
    pub fn set_input_channel_levels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings_InputChannelLevels,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_channel_levels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_channel_levels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
    type O =
        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
    #[doc = ""]
    pub output_channel: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings {
            output_channel: self.output_channel,
            input_channel_levels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_channel` after provisioning.\n"]
    pub fn output_channel(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_channel", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsDynamic {
    channel_mappings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channels_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channels_out: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_mappings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings>>,
    dynamic: MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {
    #[doc = "Set the field `channels_in`.\n"]
    pub fn set_channels_in(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.channels_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `channels_out`.\n"]
    pub fn set_channels_out(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.channels_out = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `channel_mappings`.\n"]
    pub fn set_channel_mappings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings_ChannelMappings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.channel_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.channel_mappings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {
        MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings {
            channels_in: core::default::Default::default(),
            channels_out: core::default::Default::default(),
            channel_mappings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channels_in` after provisioning.\n"]
    pub fn channels_in(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channels_in", self.base))
    }

    #[doc = "Get a reference to the value of field `channels_out` after provisioning.\n"]
    pub fn channels_out(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channels_out", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AudioDescriptionsDynamic {
    audio_normalization_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings>>,
    audio_watermark_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings>>,
    codec_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings>>,
    remix_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptions {
    audio_selector_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_type_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code_control: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_normalization_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_watermark_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec_settings: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remix_settings: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_AudioDescriptionsDynamic,
}

impl MedialiveChannel_EncoderSettings_AudioDescriptions {
    #[doc = "Set the field `audio_type`.\n"]
    pub fn set_audio_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_type_control`.\n"]
    pub fn set_audio_type_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_type_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code_control`.\n"]
    pub fn set_language_code_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_code_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_name`.\n"]
    pub fn set_stream_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_normalization_settings`.\n"]
    pub fn set_audio_normalization_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_normalization_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_normalization_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `audio_watermark_settings`.\n"]
    pub fn set_audio_watermark_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_watermark_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_watermark_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `codec_settings`.\n"]
    pub fn set_codec_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.codec_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.codec_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remix_settings`.\n"]
    pub fn set_remix_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remix_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remix_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AudioDescriptions {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AudioDescriptions {
    #[doc = ""]
    pub audio_selector_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_AudioDescriptions {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AudioDescriptions {
        MedialiveChannel_EncoderSettings_AudioDescriptions {
            audio_selector_name: self.audio_selector_name,
            audio_type: core::default::Default::default(),
            audio_type_control: core::default::Default::default(),
            language_code: core::default::Default::default(),
            language_code_control: core::default::Default::default(),
            name: self.name,
            stream_name: core::default::Default::default(),
            audio_normalization_settings: core::default::Default::default(),
            audio_watermark_settings: core::default::Default::default(),
            codec_settings: core::default::Default::default(),
            remix_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AudioDescriptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AudioDescriptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AudioDescriptionsRef {
        MedialiveChannel_EncoderSettings_AudioDescriptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AudioDescriptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_selector_name` after provisioning.\n"]
    pub fn audio_selector_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_selector_name", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_type` after provisioning.\n"]
    pub fn audio_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_type", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_type_control` after provisioning.\n"]
    pub fn audio_type_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_type_control", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code_control` after provisioning.\n"]
    pub fn language_code_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code_control", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_normalization_settings` after provisioning.\n"]
    pub fn audio_normalization_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioNormalizationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_normalization_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_watermark_settings` after provisioning.\n"]
    pub fn audio_watermark_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_AudioWatermarkSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_watermark_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `codec_settings` after provisioning.\n"]
    pub fn codec_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_CodecSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.codec_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `remix_settings` after provisioning.\n"]
    pub fn remix_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AudioDescriptions_RemixSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remix_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
        MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImageRef {
        MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_AvailBlankingDynamic {
    avail_blanking_image: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_AvailBlanking {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avail_blanking_image: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage>>,
    dynamic: MedialiveChannel_EncoderSettings_AvailBlankingDynamic,
}

impl MedialiveChannel_EncoderSettings_AvailBlanking {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avail_blanking_image`.\n"]
    pub fn set_avail_blanking_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avail_blanking_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avail_blanking_image = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_AvailBlanking {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AvailBlanking>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_AvailBlanking {}

impl BuildMedialiveChannel_EncoderSettings_AvailBlanking {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_AvailBlanking {
        MedialiveChannel_EncoderSettings_AvailBlanking {
            state: core::default::Default::default(),
            avail_blanking_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_AvailBlankingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_AvailBlankingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_AvailBlankingRef {
        MedialiveChannel_EncoderSettings_AvailBlankingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_AvailBlankingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `avail_blanking_image` after provisioning.\n"]
    pub fn avail_blanking_image(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AvailBlanking_AvailBlankingImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avail_blanking_image", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsDynamic {
    font: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alignment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_resolution: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_size: std::option::Option<terrars::PrimField<std::string::String>>,
    outline_color: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outline_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_x_offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_y_offset: std::option::Option<terrars::PrimField<f64>>,
    teletext_grid_control: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
    #[doc = "Set the field `alignment`.\n"]
    pub fn set_alignment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alignment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `background_color`.\n"]
    pub fn set_background_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `background_opacity`.\n"]
    pub fn set_background_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.background_opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_color`.\n"]
    pub fn set_font_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_opacity`.\n"]
    pub fn set_font_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.font_opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_resolution`.\n"]
    pub fn set_font_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.font_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_size`.\n"]
    pub fn set_font_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outline_size`.\n"]
    pub fn set_outline_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.outline_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_color`.\n"]
    pub fn set_shadow_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shadow_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_opacity`.\n"]
    pub fn set_shadow_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_x_offset`.\n"]
    pub fn set_shadow_x_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_x_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_y_offset`.\n"]
    pub fn set_shadow_y_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_y_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x_position`.\n"]
    pub fn set_x_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.x_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `y_position`.\n"]
    pub fn set_y_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.y_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font`.\n"]
    pub fn set_font(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.font = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.font = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
    #[doc = ""]
    pub outline_color: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub teletext_grid_control: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings {
            alignment: core::default::Default::default(),
            background_color: core::default::Default::default(),
            background_opacity: core::default::Default::default(),
            font_color: core::default::Default::default(),
            font_opacity: core::default::Default::default(),
            font_resolution: core::default::Default::default(),
            font_size: core::default::Default::default(),
            outline_color: self.outline_color,
            outline_size: core::default::Default::default(),
            shadow_color: core::default::Default::default(),
            shadow_opacity: core::default::Default::default(),
            shadow_x_offset: core::default::Default::default(),
            shadow_y_offset: core::default::Default::default(),
            teletext_grid_control: self.teletext_grid_control,
            x_position: core::default::Default::default(),
            y_position: core::default::Default::default(),
            font: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alignment` after provisioning.\n"]
    pub fn alignment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alignment", self.base))
    }

    #[doc = "Get a reference to the value of field `background_color` after provisioning.\n"]
    pub fn background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc = "Get a reference to the value of field `background_opacity` after provisioning.\n"]
    pub fn background_opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `font_color` after provisioning.\n"]
    pub fn font_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_color", self.base))
    }

    #[doc = "Get a reference to the value of field `font_opacity` after provisioning.\n"]
    pub fn font_opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `font_resolution` after provisioning.\n"]
    pub fn font_resolution(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `font_size` after provisioning.\n"]
    pub fn font_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_size", self.base))
    }

    #[doc = "Get a reference to the value of field `outline_color` after provisioning.\n"]
    pub fn outline_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outline_color", self.base))
    }

    #[doc = "Get a reference to the value of field `outline_size` after provisioning.\n"]
    pub fn outline_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outline_size", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_color` after provisioning.\n"]
    pub fn shadow_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_color", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_opacity` after provisioning.\n"]
    pub fn shadow_opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_x_offset` after provisioning.\n"]
    pub fn shadow_x_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_x_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_y_offset` after provisioning.\n"]
    pub fn shadow_y_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_y_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `teletext_grid_control` after provisioning.\n"]
    pub fn teletext_grid_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.teletext_grid_control", self.base))
    }

    #[doc = "Get a reference to the value of field `x_position` after provisioning.\n"]
    pub fn x_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x_position", self.base))
    }

    #[doc = "Get a reference to the value of field `y_position` after provisioning.\n"]
    pub fn y_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y_position", self.base))
    }

    #[doc = "Get a reference to the value of field `font` after provisioning.\n"]
    pub fn font(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.font", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsDynamic {
    font: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alignment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_resolution: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outline_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outline_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_x_offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_y_offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teletext_grid_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {
    #[doc = "Set the field `alignment`.\n"]
    pub fn set_alignment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alignment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `background_color`.\n"]
    pub fn set_background_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `background_opacity`.\n"]
    pub fn set_background_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.background_opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_color`.\n"]
    pub fn set_font_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_opacity`.\n"]
    pub fn set_font_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.font_opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_resolution`.\n"]
    pub fn set_font_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.font_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_size`.\n"]
    pub fn set_font_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outline_color`.\n"]
    pub fn set_outline_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.outline_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outline_size`.\n"]
    pub fn set_outline_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.outline_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_color`.\n"]
    pub fn set_shadow_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shadow_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_opacity`.\n"]
    pub fn set_shadow_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_x_offset`.\n"]
    pub fn set_shadow_x_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_x_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_y_offset`.\n"]
    pub fn set_shadow_y_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_y_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `teletext_grid_control`.\n"]
    pub fn set_teletext_grid_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.teletext_grid_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x_position`.\n"]
    pub fn set_x_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.x_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `y_position`.\n"]
    pub fn set_y_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.y_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font`.\n"]
    pub fn set_font(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_Font,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.font = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.font = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings {
            alignment: core::default::Default::default(),
            background_color: core::default::Default::default(),
            background_opacity: core::default::Default::default(),
            font_color: core::default::Default::default(),
            font_opacity: core::default::Default::default(),
            font_resolution: core::default::Default::default(),
            font_size: core::default::Default::default(),
            outline_color: core::default::Default::default(),
            outline_size: core::default::Default::default(),
            shadow_color: core::default::Default::default(),
            shadow_opacity: core::default::Default::default(),
            shadow_x_offset: core::default::Default::default(),
            shadow_y_offset: core::default::Default::default(),
            teletext_grid_control: core::default::Default::default(),
            x_position: core::default::Default::default(),
            y_position: core::default::Default::default(),
            font: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alignment` after provisioning.\n"]
    pub fn alignment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alignment", self.base))
    }

    #[doc = "Get a reference to the value of field `background_color` after provisioning.\n"]
    pub fn background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc = "Get a reference to the value of field `background_opacity` after provisioning.\n"]
    pub fn background_opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `font_color` after provisioning.\n"]
    pub fn font_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_color", self.base))
    }

    #[doc = "Get a reference to the value of field `font_opacity` after provisioning.\n"]
    pub fn font_opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `font_resolution` after provisioning.\n"]
    pub fn font_resolution(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `font_size` after provisioning.\n"]
    pub fn font_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_size", self.base))
    }

    #[doc = "Get a reference to the value of field `outline_color` after provisioning.\n"]
    pub fn outline_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outline_color", self.base))
    }

    #[doc = "Get a reference to the value of field `outline_size` after provisioning.\n"]
    pub fn outline_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outline_size", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_color` after provisioning.\n"]
    pub fn shadow_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_color", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_opacity` after provisioning.\n"]
    pub fn shadow_opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_x_offset` after provisioning.\n"]
    pub fn shadow_x_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_x_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_y_offset` after provisioning.\n"]
    pub fn shadow_y_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_y_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `teletext_grid_control` after provisioning.\n"]
    pub fn teletext_grid_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.teletext_grid_control", self.base))
    }

    #[doc = "Get a reference to the value of field `x_position` after provisioning.\n"]
    pub fn x_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x_position", self.base))
    }

    #[doc = "Get a reference to the value of field `y_position` after provisioning.\n"]
    pub fn y_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y_position", self.base))
    }

    #[doc = "Get a reference to the value of field `font` after provisioning.\n"]
    pub fn font(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings_FontRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.font", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copyright_holder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fill_line_gap: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    style_control: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {
    #[doc = "Set the field `copyright_holder`.\n"]
    pub fn set_copyright_holder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.copyright_holder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fill_line_gap`.\n"]
    pub fn set_fill_line_gap(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fill_line_gap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `font_family`.\n"]
    pub fn set_font_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `style_control`.\n"]
    pub fn set_style_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.style_control = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings {
            copyright_holder: core::default::Default::default(),
            fill_line_gap: core::default::Default::default(),
            font_family: core::default::Default::default(),
            style_control: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copyright_holder` after provisioning.\n"]
    pub fn copyright_holder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copyright_holder", self.base))
    }

    #[doc = "Get a reference to the value of field `fill_line_gap` after provisioning.\n"]
    pub fn fill_line_gap(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fill_line_gap", self.base))
    }

    #[doc = "Get a reference to the value of field `font_family` after provisioning.\n"]
    pub fn font_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_family", self.base))
    }

    #[doc = "Get a reference to the value of field `style_control` after provisioning.\n"]
    pub fn style_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.style_control", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings {}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings {
    style_control: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings {
    #[doc = ""]
    pub style_control: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings {
            style_control: self.style_control,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `style_control` after provisioning.\n"]
    pub fn style_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.style_control", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings {
    style_control: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings {
    #[doc = ""]
    pub style_control: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings {
            style_control: self.style_control,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `style_control` after provisioning.\n"]
    pub fn style_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.style_control", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsDynamic {
    arib_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings,
        >,
    >,
    burn_in_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings,
        >,
    >,
    dvb_sub_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings,
        >,
    >,
    ebu_tt_d_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings,
        >,
    >,
    embedded_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings,
        >,
    >,
    embedded_plus_scte20_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings,
        >,
    >,
    rtmp_caption_info_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings,
        >,
    >,
    scte20_plus_embedded_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings,
        >,
    >,
    scte27_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings,
        >,
    >,
    smpte_tt_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings,
        >,
    >,
    teletext_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings,
        >,
    >,
    ttml_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings,
        >,
    >,
    webvtt_destination_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arib_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burn_in_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_sub_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebu_tt_d_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedded_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedded_plus_scte20_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rtmp_caption_info_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte20_plus_embedded_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte27_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smpte_tt_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teletext_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttml_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webvtt_destination_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {
    #[doc = "Set the field `arib_destination_settings`.\n"]
    pub fn set_arib_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.arib_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.arib_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `burn_in_destination_settings`.\n"]
    pub fn set_burn_in_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.burn_in_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.burn_in_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dvb_sub_destination_settings`.\n"]
    pub fn set_dvb_sub_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dvb_sub_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_sub_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebu_tt_d_destination_settings`.\n"]
    pub fn set_ebu_tt_d_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebu_tt_d_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebu_tt_d_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `embedded_destination_settings`.\n"]
    pub fn set_embedded_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.embedded_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.embedded_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `embedded_plus_scte20_destination_settings`.\n"]
    pub fn set_embedded_plus_scte20_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.embedded_plus_scte20_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.embedded_plus_scte20_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rtmp_caption_info_destination_settings`.\n"]
    pub fn set_rtmp_caption_info_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rtmp_caption_info_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rtmp_caption_info_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scte20_plus_embedded_destination_settings`.\n"]
    pub fn set_scte20_plus_embedded_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scte20_plus_embedded_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scte20_plus_embedded_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scte27_destination_settings`.\n"]
    pub fn set_scte27_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scte27_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scte27_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `smpte_tt_destination_settings`.\n"]
    pub fn set_smpte_tt_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.smpte_tt_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.smpte_tt_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `teletext_destination_settings`.\n"]
    pub fn set_teletext_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.teletext_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.teletext_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ttml_destination_settings`.\n"]
    pub fn set_ttml_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ttml_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ttml_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `webvtt_destination_settings`.\n"]
    pub fn set_webvtt_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.webvtt_destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.webvtt_destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings {
            arib_destination_settings: core::default::Default::default(),
            burn_in_destination_settings: core::default::Default::default(),
            dvb_sub_destination_settings: core::default::Default::default(),
            ebu_tt_d_destination_settings: core::default::Default::default(),
            embedded_destination_settings: core::default::Default::default(),
            embedded_plus_scte20_destination_settings: core::default::Default::default(),
            rtmp_caption_info_destination_settings: core::default::Default::default(),
            scte20_plus_embedded_destination_settings: core::default::Default::default(),
            scte27_destination_settings: core::default::Default::default(),
            smpte_tt_destination_settings: core::default::Default::default(),
            teletext_destination_settings: core::default::Default::default(),
            ttml_destination_settings: core::default::Default::default(),
            webvtt_destination_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arib_destination_settings` after provisioning.\n"]
    pub fn arib_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_AribDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arib_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `burn_in_destination_settings` after provisioning.\n"]
    pub fn burn_in_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_BurnInDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.burn_in_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_sub_destination_settings` after provisioning.\n"]
    pub fn dvb_sub_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_DvbSubDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_sub_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ebu_tt_d_destination_settings` after provisioning.\n"]
    pub fn ebu_tt_d_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EbuTtDDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebu_tt_d_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `embedded_destination_settings` after provisioning.\n"]
    pub fn embedded_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedDestinationSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.embedded_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `embedded_plus_scte20_destination_settings` after provisioning.\n"]
    pub fn embedded_plus_scte20_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_EmbeddedPlusScte20DestinationSettingsRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.embedded_plus_scte20_destination_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rtmp_caption_info_destination_settings` after provisioning.\n"]
    pub fn rtmp_caption_info_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_RtmpCaptionInfoDestinationSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rtmp_caption_info_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `scte20_plus_embedded_destination_settings` after provisioning.\n"]
    pub fn scte20_plus_embedded_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte20PlusEmbeddedDestinationSettingsRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.scte20_plus_embedded_destination_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scte27_destination_settings` after provisioning.\n"]
    pub fn scte27_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_Scte27DestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scte27_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `smpte_tt_destination_settings` after provisioning.\n"]
    pub fn smpte_tt_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_SmpteTtDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smpte_tt_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `teletext_destination_settings` after provisioning.\n"]
    pub fn teletext_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TeletextDestinationSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teletext_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ttml_destination_settings` after provisioning.\n"]
    pub fn ttml_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_TtmlDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttml_destination_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `webvtt_destination_settings` after provisioning.\n"]
    pub fn webvtt_destination_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings_WebvttDestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webvtt_destination_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_CaptionDescriptionsDynamic {
    destination_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accessibility: std::option::Option<terrars::PrimField<std::string::String>>,
    caption_selector_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_CaptionDescriptionsDynamic,
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptions {
    #[doc = "Set the field `accessibility`.\n"]
    pub fn set_accessibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accessibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_description`.\n"]
    pub fn set_language_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_settings`.\n"]
    pub fn set_destination_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_CaptionDescriptions {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_CaptionDescriptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_CaptionDescriptions {
    #[doc = ""]
    pub caption_selector_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_CaptionDescriptions {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_CaptionDescriptions {
        MedialiveChannel_EncoderSettings_CaptionDescriptions {
            accessibility: core::default::Default::default(),
            caption_selector_name: self.caption_selector_name,
            language_code: core::default::Default::default(),
            language_description: core::default::Default::default(),
            name: self.name,
            destination_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_CaptionDescriptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_CaptionDescriptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_CaptionDescriptionsRef {
        MedialiveChannel_EncoderSettings_CaptionDescriptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_CaptionDescriptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accessibility` after provisioning.\n"]
    pub fn accessibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accessibility", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_selector_name` after provisioning.\n"]
    pub fn caption_selector_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caption_selector_name", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_description` after provisioning.\n"]
    pub fn language_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_settings` after provisioning.\n"]
    pub fn destination_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptions_DestinationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
        MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlateRef {
        MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorDynamic {
    input_loss_image_slate: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    black_frame_msec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_image_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_image_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repeat_frame_msec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_image_slate: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorDynamic,
}

impl MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {
    #[doc = "Set the field `black_frame_msec`.\n"]
    pub fn set_black_frame_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.black_frame_msec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_image_color`.\n"]
    pub fn set_input_loss_image_color(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.input_loss_image_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_image_type`.\n"]
    pub fn set_input_loss_image_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.input_loss_image_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repeat_frame_msec`.\n"]
    pub fn set_repeat_frame_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.repeat_frame_msec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_image_slate`.\n"]
    pub fn set_input_loss_image_slate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_loss_image_slate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_loss_image_slate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {}

impl BuildMedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {
        MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior {
            black_frame_msec: core::default::Default::default(),
            input_loss_image_color: core::default::Default::default(),
            input_loss_image_type: core::default::Default::default(),
            repeat_frame_msec: core::default::Default::default(),
            input_loss_image_slate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorRef {
        MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `black_frame_msec` after provisioning.\n"]
    pub fn black_frame_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.black_frame_msec", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_image_color` after provisioning.\n"]
    pub fn input_loss_image_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_image_color", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_image_type` after provisioning.\n"]
    pub fn input_loss_image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_image_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repeat_frame_msec` after provisioning.\n"]
    pub fn repeat_frame_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repeat_frame_msec", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_image_slate` after provisioning.\n"]
    pub fn input_loss_image_slate(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior_InputLossImageSlateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_loss_image_slate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_GlobalConfigurationDynamic {
    input_loss_behavior: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_GlobalConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_audio_gain: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_end_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_locking_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_timing_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_low_framerate_inputs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_behavior: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior>>,
    dynamic: MedialiveChannel_EncoderSettings_GlobalConfigurationDynamic,
}

impl MedialiveChannel_EncoderSettings_GlobalConfiguration {
    #[doc = "Set the field `initial_audio_gain`.\n"]
    pub fn set_initial_audio_gain(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_audio_gain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_end_action`.\n"]
    pub fn set_input_end_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_end_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_locking_mode`.\n"]
    pub fn set_output_locking_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_locking_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_timing_source`.\n"]
    pub fn set_output_timing_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_timing_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_low_framerate_inputs`.\n"]
    pub fn set_support_low_framerate_inputs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.support_low_framerate_inputs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_behavior`.\n"]
    pub fn set_input_loss_behavior(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehavior>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_loss_behavior = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_loss_behavior = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_GlobalConfiguration {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_GlobalConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_GlobalConfiguration {}

impl BuildMedialiveChannel_EncoderSettings_GlobalConfiguration {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_GlobalConfiguration {
        MedialiveChannel_EncoderSettings_GlobalConfiguration {
            initial_audio_gain: core::default::Default::default(),
            input_end_action: core::default::Default::default(),
            output_locking_mode: core::default::Default::default(),
            output_timing_source: core::default::Default::default(),
            support_low_framerate_inputs: core::default::Default::default(),
            input_loss_behavior: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_GlobalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_GlobalConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_GlobalConfigurationRef {
        MedialiveChannel_EncoderSettings_GlobalConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_GlobalConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_audio_gain` after provisioning.\n"]
    pub fn initial_audio_gain(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_audio_gain", self.base))
    }

    #[doc = "Get a reference to the value of field `input_end_action` after provisioning.\n"]
    pub fn input_end_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_end_action", self.base))
    }

    #[doc = "Get a reference to the value of field `output_locking_mode` after provisioning.\n"]
    pub fn output_locking_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_locking_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `output_timing_source` after provisioning.\n"]
    pub fn output_timing_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_timing_source", self.base))
    }

    #[doc = "Get a reference to the value of field `support_low_framerate_inputs` after provisioning.\n"]
    pub fn support_low_framerate_inputs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_low_framerate_inputs", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_behavior` after provisioning.\n"]
    pub fn input_loss_behavior(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_GlobalConfiguration_InputLossBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_loss_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {}

impl MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {}

impl BuildMedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {
        MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettingsRef {
        MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsDynamic {
    html_motion_graphics_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html_motion_graphics_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {
    #[doc = "Set the field `html_motion_graphics_settings`.\n"]
    pub fn set_html_motion_graphics_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.html_motion_graphics_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.html_motion_graphics_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {}

impl BuildMedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {
        MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings {
            html_motion_graphics_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsRef {
        MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `html_motion_graphics_settings` after provisioning.\n"]
    pub fn html_motion_graphics_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings_HtmlMotionGraphicsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.html_motion_graphics_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationDynamic {
    motion_graphics_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    motion_graphics_insertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    motion_graphics_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationDynamic,
}

impl MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {
    #[doc = "Set the field `motion_graphics_insertion`.\n"]
    pub fn set_motion_graphics_insertion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.motion_graphics_insertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `motion_graphics_settings`.\n"]
    pub fn set_motion_graphics_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.motion_graphics_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.motion_graphics_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {}

impl BuildMedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {
        MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration {
            motion_graphics_insertion: core::default::Default::default(),
            motion_graphics_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationRef {
        MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `motion_graphics_insertion` after provisioning.\n"]
    pub fn motion_graphics_insertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.motion_graphics_insertion", self.base))
    }

    #[doc = "Get a reference to the value of field `motion_graphics_settings` after provisioning.\n"]
    pub fn motion_graphics_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration_MotionGraphicsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.motion_graphics_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_NielsenConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distributor_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_pcm_to_id3_tagging: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_NielsenConfiguration {
    #[doc = "Set the field `distributor_id`.\n"]
    pub fn set_distributor_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.distributor_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nielsen_pcm_to_id3_tagging`.\n"]
    pub fn set_nielsen_pcm_to_id3_tagging(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nielsen_pcm_to_id3_tagging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_NielsenConfiguration {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_NielsenConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_NielsenConfiguration {}

impl BuildMedialiveChannel_EncoderSettings_NielsenConfiguration {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_NielsenConfiguration {
        MedialiveChannel_EncoderSettings_NielsenConfiguration {
            distributor_id: core::default::Default::default(),
            nielsen_pcm_to_id3_tagging: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_NielsenConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_NielsenConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_NielsenConfigurationRef {
        MedialiveChannel_EncoderSettings_NielsenConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_NielsenConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `distributor_id` after provisioning.\n"]
    pub fn distributor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distributor_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_pcm_to_id3_tagging` after provisioning.\n"]
    pub fn nielsen_pcm_to_id3_tagging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nielsen_pcm_to_id3_tagging", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings {
            canned_acl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3SettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsDynamic {
    archive_s3_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_s3_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {
    #[doc = "Set the field `archive_s3_settings`.\n"]
    pub fn set_archive_s3_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive_s3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_s3_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings {
            archive_s3_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_s3_settings` after provisioning.\n"]
    pub fn archive_s3_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings_ArchiveS3SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_s3_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsDynamic {
    archive_cdn_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings,
        >,
    >,
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollover_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_cdn_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {
    #[doc = "Set the field `rollover_interval`.\n"]
    pub fn set_rollover_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rollover_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `archive_cdn_settings`.\n"]
    pub fn set_archive_cdn_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive_cdn_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_cdn_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings {
            rollover_interval: core::default::Default::default(),
            archive_cdn_settings: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rollover_interval` after provisioning.\n"]
    pub fn rollover_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollover_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `archive_cdn_settings` after provisioning.\n"]
    pub fn archive_cdn_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings_ArchiveCdnSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_cdn_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings {
            canned_acl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3SettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsDynamic {
    frame_capture_s3_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_s3_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {
    #[doc = "Set the field `frame_capture_s3_settings`.\n"]
    pub fn set_frame_capture_s3_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_capture_s3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_s3_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings {
            frame_capture_s3_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frame_capture_s3_settings` after provisioning.\n"]
    pub fn frame_capture_s3_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings_FrameCaptureS3SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_s3_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    frame_capture_cdn_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_cdn_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frame_capture_cdn_settings`.\n"]
    pub fn set_frame_capture_cdn_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_capture_cdn_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_cdn_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings {
            destination: core::default::Default::default(),
            frame_capture_cdn_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_cdn_settings` after provisioning.\n"]
    pub fn frame_capture_cdn_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings_FrameCaptureCdnSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_cdn_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings {
    caption_channel: terrars::PrimField<f64>,
    language_code: terrars::PrimField<std::string::String>,
    language_description: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings {
    #[doc = ""]
    pub caption_channel: terrars::PrimField<f64>,
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub language_description: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings {
            caption_channel: self.caption_channel,
            language_code: self.language_code,
            language_description: self.language_description,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `caption_channel` after provisioning.\n"]
    pub fn caption_channel(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caption_channel", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_description` after provisioning.\n"]
    pub fn language_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_description", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filecache_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_transfer_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {
    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filecache_duration`.\n"]
    pub fn set_filecache_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filecache_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_transfer_mode`.\n"]
    pub fn set_http_transfer_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.http_transfer_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `salt`.\n"]
    pub fn set_salt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.salt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            http_transfer_mode: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
            salt: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"]
    pub fn filecache_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filecache_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `http_transfer_mode` after provisioning.\n"]
    pub fn http_transfer_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_transfer_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_delay", self.base))
    }

    #[doc = "Get a reference to the value of field `salt` after provisioning.\n"]
    pub fn salt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.salt", self.base))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filecache_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {
    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filecache_duration`.\n"]
    pub fn set_filecache_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filecache_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_delay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"]
    pub fn filecache_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filecache_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_delay", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filecache_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_store_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {
    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filecache_duration`.\n"]
    pub fn set_filecache_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filecache_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `media_store_storage_class`.\n"]
    pub fn set_media_store_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.media_store_storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_delay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            media_store_storage_class: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"]
    pub fn filecache_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filecache_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `media_store_storage_class` after provisioning.\n"]
    pub fn media_store_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.media_store_storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_delay", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings {
            canned_acl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3SettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filecache_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_transfer_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {
    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filecache_duration`.\n"]
    pub fn set_filecache_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filecache_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_transfer_mode`.\n"]
    pub fn set_http_transfer_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.http_transfer_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_delay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            http_transfer_mode: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"]
    pub fn filecache_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filecache_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `http_transfer_mode` after provisioning.\n"]
    pub fn http_transfer_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_transfer_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsDynamic {
    hls_akamai_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings,
        >,
    >,
    hls_basic_put_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings,
        >,
    >,
    hls_media_store_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings,
        >,
    >,
    hls_s3_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings,
        >,
    >,
    hls_webdav_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_akamai_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_basic_put_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_media_store_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_s3_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_webdav_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {
    #[doc = "Set the field `hls_akamai_settings`.\n"]
    pub fn set_hls_akamai_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_akamai_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_akamai_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_basic_put_settings`.\n"]
    pub fn set_hls_basic_put_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_basic_put_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_basic_put_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_media_store_settings`.\n"]
    pub fn set_hls_media_store_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_media_store_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_media_store_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_s3_settings`.\n"]
    pub fn set_hls_s3_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_s3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_s3_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_webdav_settings`.\n"]
    pub fn set_hls_webdav_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_webdav_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_webdav_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings {
            hls_akamai_settings: core::default::Default::default(),
            hls_basic_put_settings: core::default::Default::default(),
            hls_media_store_settings: core::default::Default::default(),
            hls_s3_settings: core::default::Default::default(),
            hls_webdav_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hls_akamai_settings` after provisioning.\n"]
    pub fn hls_akamai_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsAkamaiSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_akamai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_basic_put_settings` after provisioning.\n"]
    pub fn hls_basic_put_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsBasicPutSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_basic_put_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_media_store_settings` after provisioning.\n"]
    pub fn hls_media_store_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsMediaStoreSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_media_store_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_s3_settings` after provisioning.\n"]
    pub fn hls_s3_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsS3SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_s3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_webdav_settings` after provisioning.\n"]
    pub fn hls_webdav_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings_HlsWebdavSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_webdav_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServerRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsDynamic {
    key_provider_server: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
    static_key_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_provider_server: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
    #[doc = "Set the field `key_provider_server`.\n"]
    pub fn set_key_provider_server(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServer,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_provider_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_provider_server = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
    #[doc = ""]
    pub static_key_value: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings {
            static_key_value: self.static_key_value,
            key_provider_server: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_key_value` after provisioning.\n"]
    pub fn static_key_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `key_provider_server` after provisioning.\n"]
    pub fn key_provider_server(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings_KeyProviderServerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_provider_server", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsDynamic {
    static_key_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_key_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {
    #[doc = "Set the field `static_key_settings`.\n"]
    pub fn set_static_key_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_key_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_key_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings {
            static_key_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_key_settings` after provisioning.\n"]
    pub fn static_key_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings_StaticKeySettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_key_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsDynamic {
    caption_language_mappings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings,
        >,
    >,
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    hls_cdn_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings,
        >,
    >,
    key_provider_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ad_markers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url_content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url_content1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url_manifest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url_manifest1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_language_setting: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_cache: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec_specification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constant_iv: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_structure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discontinuity_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_id3_segment_tagging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iframe_only_playlists: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incomplete_segment_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_n_segments: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iv_in_manifest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iv_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_segments: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_format_versions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_duration_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_segment_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_selection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    program_date_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    program_date_time_clock: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    program_date_time_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redundant_manifest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segments_per_subdirectory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_inf_resolution: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_id3_frame: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_id3_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_delta_milliseconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ts_file_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_language_mappings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_cdn_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_provider_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {
    #[doc = "Set the field `ad_markers`.\n"]
    pub fn set_ad_markers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ad_markers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `base_url_content`.\n"]
    pub fn set_base_url_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_url_content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `base_url_content1`.\n"]
    pub fn set_base_url_content1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_url_content1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `base_url_manifest`.\n"]
    pub fn set_base_url_manifest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_url_manifest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `base_url_manifest1`.\n"]
    pub fn set_base_url_manifest1(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.base_url_manifest1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `caption_language_setting`.\n"]
    pub fn set_caption_language_setting(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.caption_language_setting = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_cache`.\n"]
    pub fn set_client_cache(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `codec_specification`.\n"]
    pub fn set_codec_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.codec_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `constant_iv`.\n"]
    pub fn set_constant_iv(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.constant_iv = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `directory_structure`.\n"]
    pub fn set_directory_structure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.directory_structure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discontinuity_tags`.\n"]
    pub fn set_discontinuity_tags(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discontinuity_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hls_id3_segment_tagging`.\n"]
    pub fn set_hls_id3_segment_tagging(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hls_id3_segment_tagging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iframe_only_playlists`.\n"]
    pub fn set_iframe_only_playlists(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.iframe_only_playlists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incomplete_segment_behavior`.\n"]
    pub fn set_incomplete_segment_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.incomplete_segment_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_n_segments`.\n"]
    pub fn set_index_n_segments(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.index_n_segments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_loss_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iv_in_manifest`.\n"]
    pub fn set_iv_in_manifest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iv_in_manifest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iv_source`.\n"]
    pub fn set_iv_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iv_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keep_segments`.\n"]
    pub fn set_keep_segments(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_segments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_format`.\n"]
    pub fn set_key_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_format_versions`.\n"]
    pub fn set_key_format_versions(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_format_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manifest_compression`.\n"]
    pub fn set_manifest_compression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.manifest_compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manifest_duration_format`.\n"]
    pub fn set_manifest_duration_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.manifest_duration_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_segment_length`.\n"]
    pub fn set_min_segment_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_segment_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_selection`.\n"]
    pub fn set_output_selection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_selection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `program_date_time`.\n"]
    pub fn set_program_date_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.program_date_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `program_date_time_clock`.\n"]
    pub fn set_program_date_time_clock(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.program_date_time_clock = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `program_date_time_period`.\n"]
    pub fn set_program_date_time_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.program_date_time_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redundant_manifest`.\n"]
    pub fn set_redundant_manifest(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redundant_manifest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_length`.\n"]
    pub fn set_segment_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.segment_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segments_per_subdirectory`.\n"]
    pub fn set_segments_per_subdirectory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.segments_per_subdirectory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_inf_resolution`.\n"]
    pub fn set_stream_inf_resolution(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.stream_inf_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_id3_frame`.\n"]
    pub fn set_timed_metadata_id3_frame(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_id3_frame = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_id3_period`.\n"]
    pub fn set_timed_metadata_id3_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timed_metadata_id3_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_delta_milliseconds`.\n"]
    pub fn set_timestamp_delta_milliseconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timestamp_delta_milliseconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ts_file_mode`.\n"]
    pub fn set_ts_file_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ts_file_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `caption_language_mappings`.\n"]
    pub fn set_caption_language_mappings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_CaptionLanguageMappings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.caption_language_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.caption_language_mappings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_cdn_settings`.\n"]
    pub fn set_hls_cdn_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_cdn_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_cdn_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `key_provider_settings`.\n"]
    pub fn set_key_provider_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_provider_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_provider_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {
    type O =
        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings {
            ad_markers: core::default::Default::default(),
            base_url_content: core::default::Default::default(),
            base_url_content1: core::default::Default::default(),
            base_url_manifest: core::default::Default::default(),
            base_url_manifest1: core::default::Default::default(),
            caption_language_setting: core::default::Default::default(),
            client_cache: core::default::Default::default(),
            codec_specification: core::default::Default::default(),
            constant_iv: core::default::Default::default(),
            directory_structure: core::default::Default::default(),
            discontinuity_tags: core::default::Default::default(),
            encryption_type: core::default::Default::default(),
            hls_id3_segment_tagging: core::default::Default::default(),
            iframe_only_playlists: core::default::Default::default(),
            incomplete_segment_behavior: core::default::Default::default(),
            index_n_segments: core::default::Default::default(),
            input_loss_action: core::default::Default::default(),
            iv_in_manifest: core::default::Default::default(),
            iv_source: core::default::Default::default(),
            keep_segments: core::default::Default::default(),
            key_format: core::default::Default::default(),
            key_format_versions: core::default::Default::default(),
            manifest_compression: core::default::Default::default(),
            manifest_duration_format: core::default::Default::default(),
            min_segment_length: core::default::Default::default(),
            mode: core::default::Default::default(),
            output_selection: core::default::Default::default(),
            program_date_time: core::default::Default::default(),
            program_date_time_clock: core::default::Default::default(),
            program_date_time_period: core::default::Default::default(),
            redundant_manifest: core::default::Default::default(),
            segment_length: core::default::Default::default(),
            segments_per_subdirectory: core::default::Default::default(),
            stream_inf_resolution: core::default::Default::default(),
            timed_metadata_id3_frame: core::default::Default::default(),
            timed_metadata_id3_period: core::default::Default::default(),
            timestamp_delta_milliseconds: core::default::Default::default(),
            ts_file_mode: core::default::Default::default(),
            caption_language_mappings: core::default::Default::default(),
            destination: core::default::Default::default(),
            hls_cdn_settings: core::default::Default::default(),
            key_provider_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ad_markers` after provisioning.\n"]
    pub fn ad_markers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ad_markers", self.base))
    }

    #[doc = "Get a reference to the value of field `base_url_content` after provisioning.\n"]
    pub fn base_url_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url_content", self.base))
    }

    #[doc = "Get a reference to the value of field `base_url_content1` after provisioning.\n"]
    pub fn base_url_content1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url_content1", self.base))
    }

    #[doc = "Get a reference to the value of field `base_url_manifest` after provisioning.\n"]
    pub fn base_url_manifest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url_manifest", self.base))
    }

    #[doc = "Get a reference to the value of field `base_url_manifest1` after provisioning.\n"]
    pub fn base_url_manifest1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url_manifest1", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_language_setting` after provisioning.\n"]
    pub fn caption_language_setting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caption_language_setting", self.base))
    }

    #[doc = "Get a reference to the value of field `client_cache` after provisioning.\n"]
    pub fn client_cache(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cache", self.base))
    }

    #[doc = "Get a reference to the value of field `codec_specification` after provisioning.\n"]
    pub fn codec_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codec_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `constant_iv` after provisioning.\n"]
    pub fn constant_iv(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constant_iv", self.base))
    }

    #[doc = "Get a reference to the value of field `directory_structure` after provisioning.\n"]
    pub fn directory_structure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_structure", self.base))
    }

    #[doc = "Get a reference to the value of field `discontinuity_tags` after provisioning.\n"]
    pub fn discontinuity_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discontinuity_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_id3_segment_tagging` after provisioning.\n"]
    pub fn hls_id3_segment_tagging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hls_id3_segment_tagging", self.base))
    }

    #[doc = "Get a reference to the value of field `iframe_only_playlists` after provisioning.\n"]
    pub fn iframe_only_playlists(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iframe_only_playlists", self.base))
    }

    #[doc = "Get a reference to the value of field `incomplete_segment_behavior` after provisioning.\n"]
    pub fn incomplete_segment_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incomplete_segment_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `index_n_segments` after provisioning.\n"]
    pub fn index_n_segments(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_n_segments", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_action", self.base))
    }

    #[doc = "Get a reference to the value of field `iv_in_manifest` after provisioning.\n"]
    pub fn iv_in_manifest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iv_in_manifest", self.base))
    }

    #[doc = "Get a reference to the value of field `iv_source` after provisioning.\n"]
    pub fn iv_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iv_source", self.base))
    }

    #[doc = "Get a reference to the value of field `keep_segments` after provisioning.\n"]
    pub fn keep_segments(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_segments", self.base))
    }

    #[doc = "Get a reference to the value of field `key_format` after provisioning.\n"]
    pub fn key_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_format", self.base))
    }

    #[doc = "Get a reference to the value of field `key_format_versions` after provisioning.\n"]
    pub fn key_format_versions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_format_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `manifest_compression` after provisioning.\n"]
    pub fn manifest_compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest_compression", self.base))
    }

    #[doc = "Get a reference to the value of field `manifest_duration_format` after provisioning.\n"]
    pub fn manifest_duration_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest_duration_format", self.base))
    }

    #[doc = "Get a reference to the value of field `min_segment_length` after provisioning.\n"]
    pub fn min_segment_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_segment_length", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `output_selection` after provisioning.\n"]
    pub fn output_selection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `program_date_time` after provisioning.\n"]
    pub fn program_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_date_time", self.base))
    }

    #[doc = "Get a reference to the value of field `program_date_time_clock` after provisioning.\n"]
    pub fn program_date_time_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_date_time_clock", self.base))
    }

    #[doc = "Get a reference to the value of field `program_date_time_period` after provisioning.\n"]
    pub fn program_date_time_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_date_time_period", self.base))
    }

    #[doc = "Get a reference to the value of field `redundant_manifest` after provisioning.\n"]
    pub fn redundant_manifest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redundant_manifest", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_length` after provisioning.\n"]
    pub fn segment_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_length", self.base))
    }

    #[doc = "Get a reference to the value of field `segments_per_subdirectory` after provisioning.\n"]
    pub fn segments_per_subdirectory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segments_per_subdirectory", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_inf_resolution` after provisioning.\n"]
    pub fn stream_inf_resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_inf_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_id3_frame` after provisioning.\n"]
    pub fn timed_metadata_id3_frame(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_id3_frame", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_id3_period` after provisioning.\n"]
    pub fn timed_metadata_id3_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_id3_period", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_delta_milliseconds` after provisioning.\n"]
    pub fn timestamp_delta_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_delta_milliseconds", self.base))
    }

    #[doc = "Get a reference to the value of field `ts_file_mode` after provisioning.\n"]
    pub fn ts_file_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ts_file_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_cdn_settings` after provisioning.\n"]
    pub fn hls_cdn_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_HlsCdnSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_cdn_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `key_provider_settings` after provisioning.\n"]
    pub fn key_provider_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_KeyProviderSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_provider_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings {
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acquisition_point_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_only_timecode_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_id_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_stop_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filecache_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fragment_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segmentation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_delay_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparse_track_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_manifest_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_offset_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {
    #[doc = "Set the field `acquisition_point_id`.\n"]
    pub fn set_acquisition_point_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.acquisition_point_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_only_timecode_control`.\n"]
    pub fn set_audio_only_timecode_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_only_timecode_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_mode`.\n"]
    pub fn set_certificate_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_id`.\n"]
    pub fn set_event_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_id_mode`.\n"]
    pub fn set_event_id_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_id_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_stop_behavior`.\n"]
    pub fn set_event_stop_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.event_stop_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filecache_duration`.\n"]
    pub fn set_filecache_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filecache_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fragment_length`.\n"]
    pub fn set_fragment_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fragment_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_loss_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segmentation_mode`.\n"]
    pub fn set_segmentation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segmentation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `send_delay_ms`.\n"]
    pub fn set_send_delay_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.send_delay_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sparse_track_type`.\n"]
    pub fn set_sparse_track_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sparse_track_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_manifest_behavior`.\n"]
    pub fn set_stream_manifest_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.stream_manifest_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_offset`.\n"]
    pub fn set_timestamp_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timestamp_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_offset_mode`.\n"]
    pub fn set_timestamp_offset_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timestamp_offset_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings {
            acquisition_point_id: core::default::Default::default(),
            audio_only_timecode_control: core::default::Default::default(),
            certificate_mode: core::default::Default::default(),
            connection_retry_interval: core::default::Default::default(),
            event_id: core::default::Default::default(),
            event_id_mode: core::default::Default::default(),
            event_stop_behavior: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            fragment_length: core::default::Default::default(),
            input_loss_action: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
            segmentation_mode: core::default::Default::default(),
            send_delay_ms: core::default::Default::default(),
            sparse_track_type: core::default::Default::default(),
            stream_manifest_behavior: core::default::Default::default(),
            timestamp_offset: core::default::Default::default(),
            timestamp_offset_mode: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acquisition_point_id` after provisioning.\n"]
    pub fn acquisition_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acquisition_point_id", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_only_timecode_control` after provisioning.\n"]
    pub fn audio_only_timecode_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_only_timecode_control", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `event_id` after provisioning.\n"]
    pub fn event_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_id", self.base))
    }

    #[doc = "Get a reference to the value of field `event_id_mode` after provisioning.\n"]
    pub fn event_id_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_id_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `event_stop_behavior` after provisioning.\n"]
    pub fn event_stop_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_stop_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `filecache_duration` after provisioning.\n"]
    pub fn filecache_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filecache_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `fragment_length` after provisioning.\n"]
    pub fn fragment_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fragment_length", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_action", self.base))
    }

    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_delay", self.base))
    }

    #[doc = "Get a reference to the value of field `segmentation_mode` after provisioning.\n"]
    pub fn segmentation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segmentation_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `send_delay_ms` after provisioning.\n"]
    pub fn send_delay_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_delay_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `sparse_track_type` after provisioning.\n"]
    pub fn sparse_track_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sparse_track_type", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_manifest_behavior` after provisioning.\n"]
    pub fn stream_manifest_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_manifest_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_offset` after provisioning.\n"]
    pub fn timestamp_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_offset_mode` after provisioning.\n"]
    pub fn timestamp_offset_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_offset_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings {}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ad_markers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_full_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {
    #[doc = "Set the field `ad_markers`.\n"]
    pub fn set_ad_markers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ad_markers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_scheme`.\n"]
    pub fn set_authentication_scheme(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_full_behavior`.\n"]
    pub fn set_cache_full_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cache_full_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_length`.\n"]
    pub fn set_cache_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cache_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `caption_data`.\n"]
    pub fn set_caption_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.caption_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_loss_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_delay`.\n"]
    pub fn set_restart_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_delay = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings {
            ad_markers: core::default::Default::default(),
            authentication_scheme: core::default::Default::default(),
            cache_full_behavior: core::default::Default::default(),
            cache_length: core::default::Default::default(),
            caption_data: core::default::Default::default(),
            input_loss_action: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ad_markers` after provisioning.\n"]
    pub fn ad_markers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ad_markers", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_scheme` after provisioning.\n"]
    pub fn authentication_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_full_behavior` after provisioning.\n"]
    pub fn cache_full_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_full_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_length` after provisioning.\n"]
    pub fn cache_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_length", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_data` after provisioning.\n"]
    pub fn caption_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caption_data", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_action", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_delay` after provisioning.\n"]
    pub fn restart_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_delay", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_id3_frame: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_id3_period: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {
    #[doc = "Set the field `input_loss_action`.\n"]
    pub fn set_input_loss_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_loss_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_id3_frame`.\n"]
    pub fn set_timed_metadata_id3_frame(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_id3_frame = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_id3_period`.\n"]
    pub fn set_timed_metadata_id3_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timed_metadata_id3_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {
    type O =
        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings {
            input_loss_action: core::default::Default::default(),
            timed_metadata_id3_frame: core::default::Default::default(),
            timed_metadata_id3_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_loss_action` after provisioning.\n"]
    pub fn input_loss_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_action", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_id3_frame` after provisioning.\n"]
    pub fn timed_metadata_id3_frame(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_id3_frame", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_id3_period` after provisioning.\n"]
    pub fn timed_metadata_id3_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_id3_period", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsDynamic {
    archive_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings>,
    >,
    frame_capture_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings>,
    >,
    hls_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings>,
    >,
    media_package_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings>,
    >,
    ms_smooth_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings>,
    >,
    multiplex_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings>,
    >,
    rtmp_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings>,
    >,
    udp_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_package_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ms_smooth_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rtmp_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {
    #[doc = "Set the field `archive_group_settings`.\n"]
    pub fn set_archive_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frame_capture_group_settings`.\n"]
    pub fn set_frame_capture_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_capture_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_group_settings`.\n"]
    pub fn set_hls_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `media_package_group_settings`.\n"]
    pub fn set_media_package_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.media_package_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.media_package_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ms_smooth_group_settings`.\n"]
    pub fn set_ms_smooth_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ms_smooth_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ms_smooth_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multiplex_group_settings`.\n"]
    pub fn set_multiplex_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multiplex_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multiplex_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rtmp_group_settings`.\n"]
    pub fn set_rtmp_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rtmp_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rtmp_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `udp_group_settings`.\n"]
    pub fn set_udp_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.udp_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.udp_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings {
            archive_group_settings: core::default::Default::default(),
            frame_capture_group_settings: core::default::Default::default(),
            hls_group_settings: core::default::Default::default(),
            media_package_group_settings: core::default::Default::default(),
            ms_smooth_group_settings: core::default::Default::default(),
            multiplex_group_settings: core::default::Default::default(),
            rtmp_group_settings: core::default::Default::default(),
            udp_group_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_group_settings` after provisioning.\n"]
    pub fn archive_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_ArchiveGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_group_settings` after provisioning.\n"]
    pub fn frame_capture_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_FrameCaptureGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_group_settings` after provisioning.\n"]
    pub fn hls_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `media_package_group_settings` after provisioning.\n"]
    pub fn media_package_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MediaPackageGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_package_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ms_smooth_group_settings` after provisioning.\n"]
    pub fn ms_smooth_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MsSmoothGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ms_smooth_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `multiplex_group_settings` after provisioning.\n"]
    pub fn multiplex_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_MultiplexGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rtmp_group_settings` after provisioning.\n"]
    pub fn rtmp_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_RtmpGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rtmp_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_group_settings` after provisioning.\n"]
    pub fn udp_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_UdpGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.udp_group_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
    network_id: terrars::PrimField<f64>,
    network_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
    #[doc = ""]
    pub network_id: terrars::PrimField<f64>,
    #[doc = ""]
    pub network_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings {
            network_id: self.network_id,
            network_name: self.network_name,
            rep_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_id` after provisioning.\n"]
    pub fn network_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_id", self.base))
    }

    #[doc = "Get a reference to the value of field `network_name` after provisioning.\n"]
    pub fn network_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_name", self.base))
    }

    #[doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"]
    pub fn rep_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rep_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_sdt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_provider_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {
    #[doc = "Set the field `output_sdt`.\n"]
    pub fn set_output_sdt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_sdt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_provider_name`.\n"]
    pub fn set_service_provider_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_provider_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings {
            output_sdt: core::default::Default::default(),
            rep_interval: core::default::Default::default(),
            service_name: core::default::Default::default(),
            service_provider_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_sdt` after provisioning.\n"]
    pub fn output_sdt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_sdt", self.base))
    }

    #[doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"]
    pub fn rep_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rep_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_provider_name` after provisioning.\n"]
    pub fn service_provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_provider_name", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {
    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings {
            rep_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"]
    pub fn rep_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rep_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsDynamic {
    dvb_nit_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings,
        >,
    >,
    dvb_sdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings,
        >,
    >,
    dvb_tdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    absent_input_audio_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arib: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arib_captions_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arib_captions_pid_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_buffer_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_frames_per_pes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_pids: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_stream_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cc_descriptor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_sub_pids: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_teletext_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebif: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebp_audio_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebp_lookahead_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebp_placement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecm_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    es_rate_in_pes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etv_platform_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etv_signal_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fragment_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    klv: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    klv_data_pids: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_id3_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    null_packet_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pat_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pcr_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pcr_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pcr_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pmt_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pmt_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    program_num: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte27_pids: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte35_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte35_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segmentation_markers: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segmentation_style: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segmentation_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport_stream_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_nit_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_sdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_tdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {
    #[doc = "Set the field `absent_input_audio_behavior`.\n"]
    pub fn set_absent_input_audio_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.absent_input_audio_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arib`.\n"]
    pub fn set_arib(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arib = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arib_captions_pid`.\n"]
    pub fn set_arib_captions_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arib_captions_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arib_captions_pid_control`.\n"]
    pub fn set_arib_captions_pid_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.arib_captions_pid_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_buffer_model`.\n"]
    pub fn set_audio_buffer_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_buffer_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_frames_per_pes`.\n"]
    pub fn set_audio_frames_per_pes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audio_frames_per_pes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_pids`.\n"]
    pub fn set_audio_pids(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_pids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_stream_type`.\n"]
    pub fn set_audio_stream_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_stream_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_model`.\n"]
    pub fn set_buffer_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.buffer_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cc_descriptor`.\n"]
    pub fn set_cc_descriptor(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cc_descriptor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dvb_sub_pids`.\n"]
    pub fn set_dvb_sub_pids(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dvb_sub_pids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dvb_teletext_pid`.\n"]
    pub fn set_dvb_teletext_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dvb_teletext_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebif`.\n"]
    pub fn set_ebif(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ebif = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebp_audio_interval`.\n"]
    pub fn set_ebp_audio_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ebp_audio_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebp_lookahead_ms`.\n"]
    pub fn set_ebp_lookahead_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ebp_lookahead_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebp_placement`.\n"]
    pub fn set_ebp_placement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ebp_placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecm_pid`.\n"]
    pub fn set_ecm_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ecm_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `es_rate_in_pes`.\n"]
    pub fn set_es_rate_in_pes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.es_rate_in_pes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etv_platform_pid`.\n"]
    pub fn set_etv_platform_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etv_platform_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etv_signal_pid`.\n"]
    pub fn set_etv_signal_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etv_signal_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fragment_time`.\n"]
    pub fn set_fragment_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fragment_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `klv`.\n"]
    pub fn set_klv(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.klv = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `klv_data_pids`.\n"]
    pub fn set_klv_data_pids(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.klv_data_pids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nielsen_id3_behavior`.\n"]
    pub fn set_nielsen_id3_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nielsen_id3_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `null_packet_bitrate`.\n"]
    pub fn set_null_packet_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.null_packet_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pat_interval`.\n"]
    pub fn set_pat_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pat_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pcr_control`.\n"]
    pub fn set_pcr_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pcr_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pcr_period`.\n"]
    pub fn set_pcr_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pcr_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pcr_pid`.\n"]
    pub fn set_pcr_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pcr_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pmt_interval`.\n"]
    pub fn set_pmt_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pmt_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pmt_pid`.\n"]
    pub fn set_pmt_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pmt_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `program_num`.\n"]
    pub fn set_program_num(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.program_num = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_mode`.\n"]
    pub fn set_rate_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rate_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte27_pids`.\n"]
    pub fn set_scte27_pids(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte27_pids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte35_control`.\n"]
    pub fn set_scte35_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte35_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte35_pid`.\n"]
    pub fn set_scte35_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte35_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segmentation_markers`.\n"]
    pub fn set_segmentation_markers(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.segmentation_markers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segmentation_style`.\n"]
    pub fn set_segmentation_style(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.segmentation_style = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segmentation_time`.\n"]
    pub fn set_segmentation_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.segmentation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_behavior`.\n"]
    pub fn set_timed_metadata_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_pid`.\n"]
    pub fn set_timed_metadata_pid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transport_stream_id`.\n"]
    pub fn set_transport_stream_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transport_stream_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_pid`.\n"]
    pub fn set_video_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.video_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dvb_nit_settings`.\n"]
    pub fn set_dvb_nit_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dvb_nit_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_nit_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dvb_sdt_settings`.\n"]
    pub fn set_dvb_sdt_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dvb_sdt_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_sdt_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dvb_tdt_settings`.\n"]
    pub fn set_dvb_tdt_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dvb_tdt_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_tdt_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings {
            absent_input_audio_behavior: core::default::Default::default(),
            arib: core::default::Default::default(),
            arib_captions_pid: core::default::Default::default(),
            arib_captions_pid_control: core::default::Default::default(),
            audio_buffer_model: core::default::Default::default(),
            audio_frames_per_pes: core::default::Default::default(),
            audio_pids: core::default::Default::default(),
            audio_stream_type: core::default::Default::default(),
            bitrate: core::default::Default::default(),
            buffer_model: core::default::Default::default(),
            cc_descriptor: core::default::Default::default(),
            dvb_sub_pids: core::default::Default::default(),
            dvb_teletext_pid: core::default::Default::default(),
            ebif: core::default::Default::default(),
            ebp_audio_interval: core::default::Default::default(),
            ebp_lookahead_ms: core::default::Default::default(),
            ebp_placement: core::default::Default::default(),
            ecm_pid: core::default::Default::default(),
            es_rate_in_pes: core::default::Default::default(),
            etv_platform_pid: core::default::Default::default(),
            etv_signal_pid: core::default::Default::default(),
            fragment_time: core::default::Default::default(),
            klv: core::default::Default::default(),
            klv_data_pids: core::default::Default::default(),
            nielsen_id3_behavior: core::default::Default::default(),
            null_packet_bitrate: core::default::Default::default(),
            pat_interval: core::default::Default::default(),
            pcr_control: core::default::Default::default(),
            pcr_period: core::default::Default::default(),
            pcr_pid: core::default::Default::default(),
            pmt_interval: core::default::Default::default(),
            pmt_pid: core::default::Default::default(),
            program_num: core::default::Default::default(),
            rate_mode: core::default::Default::default(),
            scte27_pids: core::default::Default::default(),
            scte35_control: core::default::Default::default(),
            scte35_pid: core::default::Default::default(),
            segmentation_markers: core::default::Default::default(),
            segmentation_style: core::default::Default::default(),
            segmentation_time: core::default::Default::default(),
            timed_metadata_behavior: core::default::Default::default(),
            timed_metadata_pid: core::default::Default::default(),
            transport_stream_id: core::default::Default::default(),
            video_pid: core::default::Default::default(),
            dvb_nit_settings: core::default::Default::default(),
            dvb_sdt_settings: core::default::Default::default(),
            dvb_tdt_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `absent_input_audio_behavior` after provisioning.\n"]
    pub fn absent_input_audio_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.absent_input_audio_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `arib` after provisioning.\n"]
    pub fn arib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arib", self.base))
    }

    #[doc = "Get a reference to the value of field `arib_captions_pid` after provisioning.\n"]
    pub fn arib_captions_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arib_captions_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `arib_captions_pid_control` after provisioning.\n"]
    pub fn arib_captions_pid_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arib_captions_pid_control", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_buffer_model` after provisioning.\n"]
    pub fn audio_buffer_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_buffer_model", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_frames_per_pes` after provisioning.\n"]
    pub fn audio_frames_per_pes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_frames_per_pes", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_pids` after provisioning.\n"]
    pub fn audio_pids(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_pids", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_stream_type` after provisioning.\n"]
    pub fn audio_stream_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_stream_type", self.base))
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_model` after provisioning.\n"]
    pub fn buffer_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_model", self.base))
    }

    #[doc = "Get a reference to the value of field `cc_descriptor` after provisioning.\n"]
    pub fn cc_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cc_descriptor", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_sub_pids` after provisioning.\n"]
    pub fn dvb_sub_pids(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dvb_sub_pids", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_teletext_pid` after provisioning.\n"]
    pub fn dvb_teletext_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dvb_teletext_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `ebif` after provisioning.\n"]
    pub fn ebif(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebif", self.base))
    }

    #[doc = "Get a reference to the value of field `ebp_audio_interval` after provisioning.\n"]
    pub fn ebp_audio_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebp_audio_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `ebp_lookahead_ms` after provisioning.\n"]
    pub fn ebp_lookahead_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebp_lookahead_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `ebp_placement` after provisioning.\n"]
    pub fn ebp_placement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebp_placement", self.base))
    }

    #[doc = "Get a reference to the value of field `ecm_pid` after provisioning.\n"]
    pub fn ecm_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecm_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `es_rate_in_pes` after provisioning.\n"]
    pub fn es_rate_in_pes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.es_rate_in_pes", self.base))
    }

    #[doc = "Get a reference to the value of field `etv_platform_pid` after provisioning.\n"]
    pub fn etv_platform_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etv_platform_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `etv_signal_pid` after provisioning.\n"]
    pub fn etv_signal_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etv_signal_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `fragment_time` after provisioning.\n"]
    pub fn fragment_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fragment_time", self.base))
    }

    #[doc = "Get a reference to the value of field `klv` after provisioning.\n"]
    pub fn klv(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.klv", self.base))
    }

    #[doc = "Get a reference to the value of field `klv_data_pids` after provisioning.\n"]
    pub fn klv_data_pids(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.klv_data_pids", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"]
    pub fn nielsen_id3_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nielsen_id3_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `null_packet_bitrate` after provisioning.\n"]
    pub fn null_packet_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.null_packet_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `pat_interval` after provisioning.\n"]
    pub fn pat_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pat_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `pcr_control` after provisioning.\n"]
    pub fn pcr_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pcr_control", self.base))
    }

    #[doc = "Get a reference to the value of field `pcr_period` after provisioning.\n"]
    pub fn pcr_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pcr_period", self.base))
    }

    #[doc = "Get a reference to the value of field `pcr_pid` after provisioning.\n"]
    pub fn pcr_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pcr_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `pmt_interval` after provisioning.\n"]
    pub fn pmt_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pmt_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `pmt_pid` after provisioning.\n"]
    pub fn pmt_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pmt_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `program_num` after provisioning.\n"]
    pub fn program_num(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_num", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_mode` after provisioning.\n"]
    pub fn rate_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scte27_pids` after provisioning.\n"]
    pub fn scte27_pids(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte27_pids", self.base))
    }

    #[doc = "Get a reference to the value of field `scte35_control` after provisioning.\n"]
    pub fn scte35_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte35_control", self.base))
    }

    #[doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"]
    pub fn scte35_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte35_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `segmentation_markers` after provisioning.\n"]
    pub fn segmentation_markers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segmentation_markers", self.base))
    }

    #[doc = "Get a reference to the value of field `segmentation_style` after provisioning.\n"]
    pub fn segmentation_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segmentation_style", self.base))
    }

    #[doc = "Get a reference to the value of field `segmentation_time` after provisioning.\n"]
    pub fn segmentation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segmentation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"]
    pub fn timed_metadata_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_pid` after provisioning.\n"]
    pub fn timed_metadata_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `transport_stream_id` after provisioning.\n"]
    pub fn transport_stream_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport_stream_id", self.base))
    }

    #[doc = "Get a reference to the value of field `video_pid` after provisioning.\n"]
    pub fn video_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_nit_settings` after provisioning.\n"]
    pub fn dvb_nit_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbNitSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_nit_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_sdt_settings` after provisioning.\n"]
    pub fn dvb_sdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbSdtSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_sdt_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_tdt_settings` after provisioning.\n"]
    pub fn dvb_tdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings_DvbTdtSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_tdt_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsDynamic {
    m2ts_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
        >,
    >,
    raw_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    m2ts_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {
    #[doc = "Set the field `m2ts_settings`.\n"]
    pub fn set_m2ts_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.m2ts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.m2ts_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `raw_settings`.\n"]
    pub fn set_raw_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.raw_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.raw_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings {
            m2ts_settings: core::default::Default::default(),
            raw_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `m2ts_settings` after provisioning.\n"]
    pub fn m2ts_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.m2ts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_settings` after provisioning.\n"]
    pub fn raw_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings_RawSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsDynamic {
    container_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {
    #[doc = "Set the field `extension`.\n"]
    pub fn set_extension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.extension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_modifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_settings`.\n"]
    pub fn set_container_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings {
            extension: core::default::Default::default(),
            name_modifier: core::default::Default::default(),
            container_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extension` after provisioning.\n"]
    pub fn extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extension", self.base))
    }

    #[doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"]
    pub fn name_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_modifier", self.base))
    }

    #[doc = "Get a reference to the value of field `container_settings` after provisioning.\n"]
    pub fn container_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings_ContainerSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {
    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_modifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings {
            name_modifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"]
    pub fn name_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_modifier", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImageRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsDynamic {
    audio_only_image: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_track_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_only_image: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {
    #[doc = "Set the field `audio_group_id`.\n"]
    pub fn set_audio_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_track_type`.\n"]
    pub fn set_audio_track_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_track_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_type`.\n"]
    pub fn set_segment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_only_image`.\n"]
    pub fn set_audio_only_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_only_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_only_image = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings {
            audio_group_id: core::default::Default::default(),
            audio_track_type: core::default::Default::default(),
            segment_type: core::default::Default::default(),
            audio_only_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_group_id` after provisioning.\n"]
    pub fn audio_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_track_type` after provisioning.\n"]
    pub fn audio_track_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_track_type", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_type` after provisioning.\n"]
    pub fn segment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_only_image` after provisioning.\n"]
    pub fn audio_only_image(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings_AudioOnlyImageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_only_image", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_rendition_sets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_id3_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {
    #[doc = "Set the field `audio_rendition_sets`.\n"]
    pub fn set_audio_rendition_sets(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_rendition_sets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nielsen_id3_behavior`.\n"]
    pub fn set_nielsen_id3_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nielsen_id3_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_behavior`.\n"]
    pub fn set_timed_metadata_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings {
            audio_rendition_sets: core::default::Default::default(),
            nielsen_id3_behavior: core::default::Default::default(),
            timed_metadata_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_rendition_sets` after provisioning.\n"]
    pub fn audio_rendition_sets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_rendition_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"]
    pub fn nielsen_id3_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nielsen_id3_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"]
    pub fn timed_metadata_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_frames_per_pes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_pids: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecm_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_id3_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pat_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pcr_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pcr_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pcr_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pmt_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pmt_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    program_num: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte35_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte35_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_pid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport_stream_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_pid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {
    #[doc = "Set the field `audio_frames_per_pes`.\n"]
    pub fn set_audio_frames_per_pes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audio_frames_per_pes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_pids`.\n"]
    pub fn set_audio_pids(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_pids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecm_pid`.\n"]
    pub fn set_ecm_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ecm_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nielsen_id3_behavior`.\n"]
    pub fn set_nielsen_id3_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nielsen_id3_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pat_interval`.\n"]
    pub fn set_pat_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pat_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pcr_control`.\n"]
    pub fn set_pcr_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pcr_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pcr_period`.\n"]
    pub fn set_pcr_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pcr_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pcr_pid`.\n"]
    pub fn set_pcr_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pcr_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pmt_interval`.\n"]
    pub fn set_pmt_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pmt_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pmt_pid`.\n"]
    pub fn set_pmt_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pmt_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `program_num`.\n"]
    pub fn set_program_num(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.program_num = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte35_behavior`.\n"]
    pub fn set_scte35_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte35_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte35_pid`.\n"]
    pub fn set_scte35_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte35_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_behavior`.\n"]
    pub fn set_timed_metadata_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timed_metadata_pid`.\n"]
    pub fn set_timed_metadata_pid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timed_metadata_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transport_stream_id`.\n"]
    pub fn set_transport_stream_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transport_stream_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_pid`.\n"]
    pub fn set_video_pid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.video_pid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings {
            audio_frames_per_pes: core::default::Default::default(),
            audio_pids: core::default::Default::default(),
            ecm_pid: core::default::Default::default(),
            nielsen_id3_behavior: core::default::Default::default(),
            pat_interval: core::default::Default::default(),
            pcr_control: core::default::Default::default(),
            pcr_period: core::default::Default::default(),
            pcr_pid: core::default::Default::default(),
            pmt_interval: core::default::Default::default(),
            pmt_pid: core::default::Default::default(),
            program_num: core::default::Default::default(),
            scte35_behavior: core::default::Default::default(),
            scte35_pid: core::default::Default::default(),
            timed_metadata_behavior: core::default::Default::default(),
            timed_metadata_pid: core::default::Default::default(),
            transport_stream_id: core::default::Default::default(),
            video_pid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8SettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_frames_per_pes` after provisioning.\n"]
    pub fn audio_frames_per_pes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_frames_per_pes", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_pids` after provisioning.\n"]
    pub fn audio_pids(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_pids", self.base))
    }

    #[doc = "Get a reference to the value of field `ecm_pid` after provisioning.\n"]
    pub fn ecm_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecm_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_id3_behavior` after provisioning.\n"]
    pub fn nielsen_id3_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nielsen_id3_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `pat_interval` after provisioning.\n"]
    pub fn pat_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pat_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `pcr_control` after provisioning.\n"]
    pub fn pcr_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pcr_control", self.base))
    }

    #[doc = "Get a reference to the value of field `pcr_period` after provisioning.\n"]
    pub fn pcr_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pcr_period", self.base))
    }

    #[doc = "Get a reference to the value of field `pcr_pid` after provisioning.\n"]
    pub fn pcr_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pcr_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `pmt_interval` after provisioning.\n"]
    pub fn pmt_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pmt_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `pmt_pid` after provisioning.\n"]
    pub fn pmt_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pmt_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `program_num` after provisioning.\n"]
    pub fn program_num(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_num", self.base))
    }

    #[doc = "Get a reference to the value of field `scte35_behavior` after provisioning.\n"]
    pub fn scte35_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte35_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"]
    pub fn scte35_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte35_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_behavior` after provisioning.\n"]
    pub fn timed_metadata_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_metadata_pid` after provisioning.\n"]
    pub fn timed_metadata_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timed_metadata_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `transport_stream_id` after provisioning.\n"]
    pub fn transport_stream_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport_stream_id", self.base))
    }

    #[doc = "Get a reference to the value of field `video_pid` after provisioning.\n"]
    pub fn video_pid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_pid", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsDynamic {
    m3u8_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_rendition_sets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    m3u8_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {
    #[doc = "Set the field `audio_rendition_sets`.\n"]
    pub fn set_audio_rendition_sets(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_rendition_sets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `m3u8_settings`.\n"]
    pub fn set_m3u8_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.m3u8_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.m3u8_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings {
            audio_rendition_sets: core::default::Default::default(),
            m3u8_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_rendition_sets` after provisioning.\n"]
    pub fn audio_rendition_sets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_rendition_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `m3u8_settings` after provisioning.\n"]
    pub fn m3u8_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings_M3u8SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.m3u8_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsDynamic {
    audio_only_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings,
        >,
    >,
    fmp4_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings,
        >,
    >,
    frame_capture_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings,
        >,
    >,
    standard_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_only_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fmp4_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {
    #[doc = "Set the field `audio_only_hls_settings`.\n"]
    pub fn set_audio_only_hls_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_only_hls_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_only_hls_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fmp4_hls_settings`.\n"]
    pub fn set_fmp4_hls_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fmp4_hls_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fmp4_hls_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frame_capture_hls_settings`.\n"]
    pub fn set_frame_capture_hls_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_capture_hls_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_hls_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `standard_hls_settings`.\n"]
    pub fn set_standard_hls_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.standard_hls_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_hls_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings {
            audio_only_hls_settings: core::default::Default::default(),
            fmp4_hls_settings: core::default::Default::default(),
            frame_capture_hls_settings: core::default::Default::default(),
            standard_hls_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_only_hls_settings` after provisioning.\n"]
    pub fn audio_only_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_AudioOnlyHlsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_only_hls_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `fmp4_hls_settings` after provisioning.\n"]
    pub fn fmp4_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_Fmp4HlsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fmp4_hls_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_hls_settings` after provisioning.\n"]
    pub fn frame_capture_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_FrameCaptureHlsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_hls_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_hls_settings` after provisioning.\n"]
    pub fn standard_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings_StandardHlsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_hls_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsDynamic {
    hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h265_packaging_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {
    #[doc = "Set the field `h265_packaging_type`.\n"]
    pub fn set_h265_packaging_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.h265_packaging_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_modifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_modifier`.\n"]
    pub fn set_segment_modifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment_modifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hls_settings`.\n"]
    pub fn set_hls_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings {
            h265_packaging_type: core::default::Default::default(),
            name_modifier: core::default::Default::default(),
            segment_modifier: core::default::Default::default(),
            hls_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `h265_packaging_type` after provisioning.\n"]
    pub fn h265_packaging_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.h265_packaging_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"]
    pub fn name_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_modifier", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_modifier` after provisioning.\n"]
    pub fn segment_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_modifier", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_settings` after provisioning.\n"]
    pub fn hls_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings_HlsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings {}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings { }

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h265_packaging_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {
    #[doc = "Set the field `h265_packaging_type`.\n"]
    pub fn set_h265_packaging_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.h265_packaging_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_modifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings {
            h265_packaging_type: core::default::Default::default(),
            name_modifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `h265_packaging_type` after provisioning.\n"]
    pub fn h265_packaging_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.h265_packaging_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"]
    pub fn name_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_modifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings {
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {
    #[doc = "Set the field `certificate_mode`.\n"]
    pub fn set_certificate_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_retry_interval`.\n"]
    pub fn set_connection_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_retries`.\n"]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings {
            certificate_mode: core::default::Default::default(),
            connection_retry_interval: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_retry_interval` after provisioning.\n"]
    pub fn connection_retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\n"]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsDynamic {
    m2ts_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    m2ts_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {
    #[doc = "Set the field `m2ts_settings`.\n"]
    pub fn set_m2ts_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.m2ts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.m2ts_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings {
            m2ts_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `m2ts_settings` after provisioning.\n"]
    pub fn m2ts_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings_M2tsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.m2ts_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_fec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_length: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {
    #[doc = "Set the field `column_depth`.\n"]
    pub fn set_column_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.column_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_fec`.\n"]
    pub fn set_include_fec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.include_fec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_length`.\n"]
    pub fn set_row_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings {
            column_depth: core::default::Default::default(),
            include_fec: core::default::Default::default(),
            row_length: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_depth` after provisioning.\n"]
    pub fn column_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `include_fec` after provisioning.\n"]
    pub fn include_fec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_fec", self.base))
    }

    #[doc = "Get a reference to the value of field `row_length` after provisioning.\n"]
    pub fn row_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_length", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsDynamic {
    container_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings,
        >,
    >,
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    fec_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_msec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fec_output_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {
    #[doc = "Set the field `buffer_msec`.\n"]
    pub fn set_buffer_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_msec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_settings`.\n"]
    pub fn set_container_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fec_output_settings`.\n"]
    pub fn set_fec_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fec_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fec_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings {
            buffer_msec: core::default::Default::default(),
            container_settings: core::default::Default::default(),
            destination: core::default::Default::default(),
            fec_output_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buffer_msec` after provisioning.\n"]
    pub fn buffer_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_msec", self.base))
    }

    #[doc = "Get a reference to the value of field `container_settings` after provisioning.\n"]
    pub fn container_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_ContainerSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings_HlsGroupSettings_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `fec_output_settings` after provisioning.\n"]
    pub fn fec_output_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings_FecOutputSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fec_output_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsDynamic {
    archive_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings>,
    >,
    frame_capture_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings,
        >,
    >,
    hls_output_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings>,
    >,
    media_package_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings,
        >,
    >,
    ms_smooth_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings>,
    >,
    multiplex_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings>,
    >,
    rtmp_output_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings>,
    >,
    udp_output_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_output_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_package_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ms_smooth_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rtmp_output_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_output_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {
    #[doc = "Set the field `archive_output_settings`.\n"]
    pub fn set_archive_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frame_capture_output_settings`.\n"]
    pub fn set_frame_capture_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_capture_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hls_output_settings`.\n"]
    pub fn set_hls_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `media_package_output_settings`.\n"]
    pub fn set_media_package_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.media_package_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.media_package_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ms_smooth_output_settings`.\n"]
    pub fn set_ms_smooth_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ms_smooth_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ms_smooth_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multiplex_output_settings`.\n"]
    pub fn set_multiplex_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multiplex_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multiplex_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rtmp_output_settings`.\n"]
    pub fn set_rtmp_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rtmp_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rtmp_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `udp_output_settings`.\n"]
    pub fn set_udp_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.udp_output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.udp_output_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings {
            archive_output_settings: core::default::Default::default(),
            frame_capture_output_settings: core::default::Default::default(),
            hls_output_settings: core::default::Default::default(),
            media_package_output_settings: core::default::Default::default(),
            ms_smooth_output_settings: core::default::Default::default(),
            multiplex_output_settings: core::default::Default::default(),
            rtmp_output_settings: core::default::Default::default(),
            udp_output_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_output_settings` after provisioning.\n"]
    pub fn archive_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_ArchiveOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_output_settings` after provisioning.\n"]
    pub fn frame_capture_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_FrameCaptureOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_output_settings` after provisioning.\n"]
    pub fn hls_output_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_HlsOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `media_package_output_settings` after provisioning.\n"]
    pub fn media_package_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MediaPackageOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_package_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ms_smooth_output_settings` after provisioning.\n"]
    pub fn ms_smooth_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MsSmoothOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ms_smooth_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `multiplex_output_settings` after provisioning.\n"]
    pub fn multiplex_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_MultiplexOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rtmp_output_settings` after provisioning.\n"]
    pub fn rtmp_output_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_RtmpOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rtmp_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_output_settings` after provisioning.\n"]
    pub fn udp_output_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings_UdpOutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.udp_output_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroups_OutputsDynamic {
    output_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_Outputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_description_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_description_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_description_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroups_OutputsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups_Outputs {
    #[doc = "Set the field `audio_description_names`.\n"]
    pub fn set_audio_description_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.audio_description_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `caption_description_names`.\n"]
    pub fn set_caption_description_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.caption_description_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_name`.\n"]
    pub fn set_output_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_description_name`.\n"]
    pub fn set_video_description_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.video_description_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_settings`.\n"]
    pub fn set_output_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups_Outputs {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups_Outputs {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups_Outputs {
        MedialiveChannel_EncoderSettings_OutputGroups_Outputs {
            audio_description_names: core::default::Default::default(),
            caption_description_names: core::default::Default::default(),
            output_name: core::default::Default::default(),
            video_description_name: core::default::Default::default(),
            output_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups_OutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroups_OutputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroups_OutputsRef {
        MedialiveChannel_EncoderSettings_OutputGroups_OutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroups_OutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_description_names` after provisioning.\n"]
    pub fn audio_description_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.audio_description_names", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_description_names` after provisioning.\n"]
    pub fn caption_description_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.caption_description_names", self.base))
    }

    #[doc = "Get a reference to the value of field `output_name` after provisioning.\n"]
    pub fn output_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_name", self.base))
    }

    #[doc = "Get a reference to the value of field `video_description_name` after provisioning.\n"]
    pub fn video_description_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_description_name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_settings` after provisioning.\n"]
    pub fn output_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_Outputs_OutputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_OutputGroupsDynamic {
    output_group_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings>>,
    outputs: std::option::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups_Outputs>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_OutputGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outputs: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups_Outputs>>,
    dynamic: MedialiveChannel_EncoderSettings_OutputGroupsDynamic,
}

impl MedialiveChannel_EncoderSettings_OutputGroups {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_group_settings`.\n"]
    pub fn set_output_group_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_group_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_group_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outputs`.\n"]
    pub fn set_outputs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups_Outputs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.outputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.outputs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_OutputGroups {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_OutputGroups {}

impl BuildMedialiveChannel_EncoderSettings_OutputGroups {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_OutputGroups {
        MedialiveChannel_EncoderSettings_OutputGroups {
            name: core::default::Default::default(),
            output_group_settings: core::default::Default::default(),
            outputs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_OutputGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_OutputGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_OutputGroupsRef {
        MedialiveChannel_EncoderSettings_OutputGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_OutputGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_group_settings` after provisioning.\n"]
    pub fn output_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputGroupSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroups_OutputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_TimecodeConfig {
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_TimecodeConfig {
    #[doc = "Set the field `sync_threshold`.\n"]
    pub fn set_sync_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sync_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_TimecodeConfig {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_TimecodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_TimecodeConfig {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_TimecodeConfig {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_TimecodeConfig {
        MedialiveChannel_EncoderSettings_TimecodeConfig {
            source: self.source,
            sync_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_TimecodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_TimecodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_TimecodeConfigRef {
        MedialiveChannel_EncoderSettings_TimecodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_TimecodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_threshold` after provisioning.\n"]
    pub fn sync_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_interval_units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {
    #[doc = "Set the field `capture_interval`.\n"]
    pub fn set_capture_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.capture_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capture_interval_units`.\n"]
    pub fn set_capture_interval_units(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capture_interval_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings {
            capture_interval: core::default::Default::default(),
            capture_interval_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capture_interval` after provisioning.\n"]
    pub fn capture_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capture_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `capture_interval_units` after provisioning.\n"]
    pub fn capture_interval_units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capture_interval_units", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_filter_sharpening: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strength: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {
    #[doc = "Set the field `post_filter_sharpening`.\n"]
    pub fn set_post_filter_sharpening(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_filter_sharpening = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strength`.\n"]
    pub fn set_strength(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strength = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings {
            post_filter_sharpening: core::default::Default::default(),
            strength: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `post_filter_sharpening` after provisioning.\n"]
    pub fn post_filter_sharpening(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_filter_sharpening", self.base))
    }

    #[doc = "Get a reference to the value of field `strength` after provisioning.\n"]
    pub fn strength(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strength", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsDynamic {
    temporal_filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporal_filter_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {
    #[doc = "Set the field `temporal_filter_settings`.\n"]
    pub fn set_temporal_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.temporal_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.temporal_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings {
            temporal_filter_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `temporal_filter_settings` after provisioning.\n"]
    pub fn temporal_filter_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings_TemporalFilterSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.temporal_filter_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsDynamic {
    filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adaptive_quantization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    afd_signaling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buf_fill_pct: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buf_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_metadata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entropy_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_afd: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flicker_aq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_field_pictures: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framerate_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framerate_denominator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framerate_numerator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_b_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_closed_cadence: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_num_b_frames: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_size_units: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    look_ahead_rate_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_i_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_ref_frames: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    par_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    par_denominator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    par_numerator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quality_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qvbr_quality_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_control_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scene_change_detect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slices: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    softness: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spatial_aq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subgop_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    syntax: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporal_aq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_insertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {
    #[doc = "Set the field `adaptive_quantization`.\n"]
    pub fn set_adaptive_quantization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.adaptive_quantization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `afd_signaling`.\n"]
    pub fn set_afd_signaling(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.afd_signaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitrate`.\n"]
    pub fn set_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buf_fill_pct`.\n"]
    pub fn set_buf_fill_pct(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buf_fill_pct = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buf_size`.\n"]
    pub fn set_buf_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buf_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color_metadata`.\n"]
    pub fn set_color_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entropy_encoding`.\n"]
    pub fn set_entropy_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entropy_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_afd`.\n"]
    pub fn set_fixed_afd(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_afd = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flicker_aq`.\n"]
    pub fn set_flicker_aq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flicker_aq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_field_pictures`.\n"]
    pub fn set_force_field_pictures(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.force_field_pictures = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framerate_control`.\n"]
    pub fn set_framerate_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.framerate_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framerate_denominator`.\n"]
    pub fn set_framerate_denominator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.framerate_denominator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framerate_numerator`.\n"]
    pub fn set_framerate_numerator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.framerate_numerator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_b_reference`.\n"]
    pub fn set_gop_b_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gop_b_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_closed_cadence`.\n"]
    pub fn set_gop_closed_cadence(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gop_closed_cadence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_num_b_frames`.\n"]
    pub fn set_gop_num_b_frames(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gop_num_b_frames = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_size`.\n"]
    pub fn set_gop_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gop_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_size_units`.\n"]
    pub fn set_gop_size_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gop_size_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `look_ahead_rate_control`.\n"]
    pub fn set_look_ahead_rate_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.look_ahead_rate_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_bitrate`.\n"]
    pub fn set_max_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_i_interval`.\n"]
    pub fn set_min_i_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_i_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_ref_frames`.\n"]
    pub fn set_num_ref_frames(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_ref_frames = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `par_control`.\n"]
    pub fn set_par_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.par_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `par_denominator`.\n"]
    pub fn set_par_denominator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.par_denominator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `par_numerator`.\n"]
    pub fn set_par_numerator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.par_numerator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quality_level`.\n"]
    pub fn set_quality_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quality_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qvbr_quality_level`.\n"]
    pub fn set_qvbr_quality_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.qvbr_quality_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_control_mode`.\n"]
    pub fn set_rate_control_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rate_control_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_type`.\n"]
    pub fn set_scan_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scan_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scene_change_detect`.\n"]
    pub fn set_scene_change_detect(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scene_change_detect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slices`.\n"]
    pub fn set_slices(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.slices = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `softness`.\n"]
    pub fn set_softness(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.softness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spatial_aq`.\n"]
    pub fn set_spatial_aq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spatial_aq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subgop_length`.\n"]
    pub fn set_subgop_length(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subgop_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `syntax`.\n"]
    pub fn set_syntax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.syntax = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temporal_aq`.\n"]
    pub fn set_temporal_aq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temporal_aq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timecode_insertion`.\n"]
    pub fn set_timecode_insertion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timecode_insertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_settings`.\n"]
    pub fn set_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings {
            adaptive_quantization: core::default::Default::default(),
            afd_signaling: core::default::Default::default(),
            bitrate: core::default::Default::default(),
            buf_fill_pct: core::default::Default::default(),
            buf_size: core::default::Default::default(),
            color_metadata: core::default::Default::default(),
            entropy_encoding: core::default::Default::default(),
            fixed_afd: core::default::Default::default(),
            flicker_aq: core::default::Default::default(),
            force_field_pictures: core::default::Default::default(),
            framerate_control: core::default::Default::default(),
            framerate_denominator: core::default::Default::default(),
            framerate_numerator: core::default::Default::default(),
            gop_b_reference: core::default::Default::default(),
            gop_closed_cadence: core::default::Default::default(),
            gop_num_b_frames: core::default::Default::default(),
            gop_size: core::default::Default::default(),
            gop_size_units: core::default::Default::default(),
            level: core::default::Default::default(),
            look_ahead_rate_control: core::default::Default::default(),
            max_bitrate: core::default::Default::default(),
            min_i_interval: core::default::Default::default(),
            num_ref_frames: core::default::Default::default(),
            par_control: core::default::Default::default(),
            par_denominator: core::default::Default::default(),
            par_numerator: core::default::Default::default(),
            profile: core::default::Default::default(),
            quality_level: core::default::Default::default(),
            qvbr_quality_level: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            scan_type: core::default::Default::default(),
            scene_change_detect: core::default::Default::default(),
            slices: core::default::Default::default(),
            softness: core::default::Default::default(),
            spatial_aq: core::default::Default::default(),
            subgop_length: core::default::Default::default(),
            syntax: core::default::Default::default(),
            temporal_aq: core::default::Default::default(),
            timecode_insertion: core::default::Default::default(),
            filter_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adaptive_quantization` after provisioning.\n"]
    pub fn adaptive_quantization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adaptive_quantization", self.base))
    }

    #[doc = "Get a reference to the value of field `afd_signaling` after provisioning.\n"]
    pub fn afd_signaling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.afd_signaling", self.base))
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `buf_fill_pct` after provisioning.\n"]
    pub fn buf_fill_pct(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buf_fill_pct", self.base))
    }

    #[doc = "Get a reference to the value of field `buf_size` after provisioning.\n"]
    pub fn buf_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buf_size", self.base))
    }

    #[doc = "Get a reference to the value of field `color_metadata` after provisioning.\n"]
    pub fn color_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `entropy_encoding` after provisioning.\n"]
    pub fn entropy_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entropy_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_afd` after provisioning.\n"]
    pub fn fixed_afd(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_afd", self.base))
    }

    #[doc = "Get a reference to the value of field `flicker_aq` after provisioning.\n"]
    pub fn flicker_aq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flicker_aq", self.base))
    }

    #[doc = "Get a reference to the value of field `force_field_pictures` after provisioning.\n"]
    pub fn force_field_pictures(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_field_pictures", self.base))
    }

    #[doc = "Get a reference to the value of field `framerate_control` after provisioning.\n"]
    pub fn framerate_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framerate_control", self.base))
    }

    #[doc = "Get a reference to the value of field `framerate_denominator` after provisioning.\n"]
    pub fn framerate_denominator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framerate_denominator", self.base))
    }

    #[doc = "Get a reference to the value of field `framerate_numerator` after provisioning.\n"]
    pub fn framerate_numerator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framerate_numerator", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_b_reference` after provisioning.\n"]
    pub fn gop_b_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_b_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_closed_cadence` after provisioning.\n"]
    pub fn gop_closed_cadence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_closed_cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_num_b_frames` after provisioning.\n"]
    pub fn gop_num_b_frames(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_num_b_frames", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_size` after provisioning.\n"]
    pub fn gop_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_size_units` after provisioning.\n"]
    pub fn gop_size_units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_size_units", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc = "Get a reference to the value of field `look_ahead_rate_control` after provisioning.\n"]
    pub fn look_ahead_rate_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.look_ahead_rate_control", self.base))
    }

    #[doc = "Get a reference to the value of field `max_bitrate` after provisioning.\n"]
    pub fn max_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `min_i_interval` after provisioning.\n"]
    pub fn min_i_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_i_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `num_ref_frames` after provisioning.\n"]
    pub fn num_ref_frames(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_ref_frames", self.base))
    }

    #[doc = "Get a reference to the value of field `par_control` after provisioning.\n"]
    pub fn par_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.par_control", self.base))
    }

    #[doc = "Get a reference to the value of field `par_denominator` after provisioning.\n"]
    pub fn par_denominator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.par_denominator", self.base))
    }

    #[doc = "Get a reference to the value of field `par_numerator` after provisioning.\n"]
    pub fn par_numerator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.par_numerator", self.base))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }

    #[doc = "Get a reference to the value of field `quality_level` after provisioning.\n"]
    pub fn quality_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quality_level", self.base))
    }

    #[doc = "Get a reference to the value of field `qvbr_quality_level` after provisioning.\n"]
    pub fn qvbr_quality_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qvbr_quality_level", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\n"]
    pub fn rate_control_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_control_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scene_change_detect` after provisioning.\n"]
    pub fn scene_change_detect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scene_change_detect", self.base))
    }

    #[doc = "Get a reference to the value of field `slices` after provisioning.\n"]
    pub fn slices(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slices", self.base))
    }

    #[doc = "Get a reference to the value of field `softness` after provisioning.\n"]
    pub fn softness(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.softness", self.base))
    }

    #[doc = "Get a reference to the value of field `spatial_aq` after provisioning.\n"]
    pub fn spatial_aq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spatial_aq", self.base))
    }

    #[doc = "Get a reference to the value of field `subgop_length` after provisioning.\n"]
    pub fn subgop_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subgop_length", self.base))
    }

    #[doc = "Get a reference to the value of field `syntax` after provisioning.\n"]
    pub fn syntax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.syntax", self.base))
    }

    #[doc = "Get a reference to the value of field `temporal_aq` after provisioning.\n"]
    pub fn temporal_aq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporal_aq", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_insertion` after provisioning.\n"]
    pub fn timecode_insertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timecode_insertion", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_settings` after provisioning.\n"]
    pub fn filter_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81SettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cll: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_fall: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {
    #[doc = "Set the field `max_cll`.\n"]
    pub fn set_max_cll(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cll = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_fall`.\n"]
    pub fn set_max_fall(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_fall = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings {
            max_cll: core::default::Default::default(),
            max_fall: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10SettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_cll` after provisioning.\n"]
    pub fn max_cll(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cll", self.base))
    }

    #[doc = "Get a reference to the value of field `max_fall` after provisioning.\n"]
    pub fn max_fall(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_fall", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601SettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {

}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709SettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsDynamic {
    color_space_passthrough_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings,
        >,
    >,
    dolby_vision81_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings,
        >,
    >,
    hdr10_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings,
        >,
    >,
    rec601_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings,
        >,
    >,
    rec709_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_space_passthrough_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dolby_vision81_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hdr10_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rec601_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rec709_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings,
        >,
    >,
    dynamic: MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {
    #[doc = "Set the field `color_space_passthrough_settings`.\n"]
    pub fn set_color_space_passthrough_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.color_space_passthrough_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.color_space_passthrough_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dolby_vision81_settings`.\n"]
    pub fn set_dolby_vision81_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dolby_vision81_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dolby_vision81_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hdr10_settings`.\n"]
    pub fn set_hdr10_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hdr10_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hdr10_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rec601_settings`.\n"]
    pub fn set_rec601_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rec601_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rec601_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rec709_settings`.\n"]
    pub fn set_rec709_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rec709_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rec709_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings {
            color_space_passthrough_settings: core::default::Default::default(),
            dolby_vision81_settings: core::default::Default::default(),
            hdr10_settings: core::default::Default::default(),
            rec601_settings: core::default::Default::default(),
            rec709_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color_space_passthrough_settings` after provisioning.\n"]
    pub fn color_space_passthrough_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_ColorSpacePassthroughSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.color_space_passthrough_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dolby_vision81_settings` after provisioning.\n"]
    pub fn dolby_vision81_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_DolbyVision81SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dolby_vision81_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hdr10_settings` after provisioning.\n"]
    pub fn hdr10_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Hdr10SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hdr10_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rec601_settings` after provisioning.\n"]
    pub fn rec601_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec601SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rec601_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rec709_settings` after provisioning.\n"]
    pub fn rec709_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings_Rec709SettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rec709_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_burnin_font_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_burnin_position: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timecode_burnin_font_size`.\n"]
    pub fn set_timecode_burnin_font_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timecode_burnin_font_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timecode_burnin_position`.\n"]
    pub fn set_timecode_burnin_position(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timecode_burnin_position = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings {
            prefix: core::default::Default::default(),
            timecode_burnin_font_size: core::default::Default::default(),
            timecode_burnin_position: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_burnin_font_size` after provisioning.\n"]
    pub fn timecode_burnin_font_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timecode_burnin_font_size", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_burnin_position` after provisioning.\n"]
    pub fn timecode_burnin_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timecode_burnin_position", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsDynamic {
    color_space_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings,
        >,
    >,
    filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings>,
    >,
    timecode_burnin_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adaptive_quantization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    afd_signaling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_transfer_function: std::option::Option<terrars::PrimField<std::string::String>>,
    bitrate: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buf_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_metadata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_afd: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flicker_aq: std::option::Option<terrars::PrimField<std::string::String>>,
    framerate_denominator: terrars::PrimField<f64>,
    framerate_numerator: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_closed_cadence: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gop_size_units: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    look_ahead_rate_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_i_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_qp: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mv_over_picture_boundaries: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mv_temporal_predictor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    par_denominator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    par_numerator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qvbr_quality_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_control_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scene_change_detect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slices: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile_height: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile_padding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile_width: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_insertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    treeblock_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_space_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_burnin_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings>,
    >,
    dynamic: MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
    #[doc = "Set the field `adaptive_quantization`.\n"]
    pub fn set_adaptive_quantization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.adaptive_quantization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `afd_signaling`.\n"]
    pub fn set_afd_signaling(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.afd_signaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alternative_transfer_function`.\n"]
    pub fn set_alternative_transfer_function(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.alternative_transfer_function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buf_size`.\n"]
    pub fn set_buf_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buf_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color_metadata`.\n"]
    pub fn set_color_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_afd`.\n"]
    pub fn set_fixed_afd(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_afd = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flicker_aq`.\n"]
    pub fn set_flicker_aq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flicker_aq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_closed_cadence`.\n"]
    pub fn set_gop_closed_cadence(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gop_closed_cadence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_size`.\n"]
    pub fn set_gop_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gop_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gop_size_units`.\n"]
    pub fn set_gop_size_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gop_size_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `look_ahead_rate_control`.\n"]
    pub fn set_look_ahead_rate_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.look_ahead_rate_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_bitrate`.\n"]
    pub fn set_max_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_i_interval`.\n"]
    pub fn set_min_i_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_i_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_qp`.\n"]
    pub fn set_min_qp(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_qp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mv_over_picture_boundaries`.\n"]
    pub fn set_mv_over_picture_boundaries(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mv_over_picture_boundaries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mv_temporal_predictor`.\n"]
    pub fn set_mv_temporal_predictor(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mv_temporal_predictor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `par_denominator`.\n"]
    pub fn set_par_denominator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.par_denominator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `par_numerator`.\n"]
    pub fn set_par_numerator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.par_numerator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qvbr_quality_level`.\n"]
    pub fn set_qvbr_quality_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.qvbr_quality_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_control_mode`.\n"]
    pub fn set_rate_control_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rate_control_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_type`.\n"]
    pub fn set_scan_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scan_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scene_change_detect`.\n"]
    pub fn set_scene_change_detect(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scene_change_detect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slices`.\n"]
    pub fn set_slices(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.slices = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tile_height`.\n"]
    pub fn set_tile_height(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tile_height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tile_padding`.\n"]
    pub fn set_tile_padding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tile_padding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tile_width`.\n"]
    pub fn set_tile_width(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tile_width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timecode_insertion`.\n"]
    pub fn set_timecode_insertion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timecode_insertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `treeblock_size`.\n"]
    pub fn set_treeblock_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.treeblock_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color_space_settings`.\n"]
    pub fn set_color_space_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.color_space_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.color_space_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_settings`.\n"]
    pub fn set_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timecode_burnin_settings`.\n"]
    pub fn set_timecode_burnin_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timecode_burnin_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timecode_burnin_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
    #[doc = ""]
    pub bitrate: terrars::PrimField<f64>,
    #[doc = ""]
    pub framerate_denominator: terrars::PrimField<f64>,
    #[doc = ""]
    pub framerate_numerator: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings {
            adaptive_quantization: core::default::Default::default(),
            afd_signaling: core::default::Default::default(),
            alternative_transfer_function: core::default::Default::default(),
            bitrate: self.bitrate,
            buf_size: core::default::Default::default(),
            color_metadata: core::default::Default::default(),
            fixed_afd: core::default::Default::default(),
            flicker_aq: core::default::Default::default(),
            framerate_denominator: self.framerate_denominator,
            framerate_numerator: self.framerate_numerator,
            gop_closed_cadence: core::default::Default::default(),
            gop_size: core::default::Default::default(),
            gop_size_units: core::default::Default::default(),
            level: core::default::Default::default(),
            look_ahead_rate_control: core::default::Default::default(),
            max_bitrate: core::default::Default::default(),
            min_i_interval: core::default::Default::default(),
            min_qp: core::default::Default::default(),
            mv_over_picture_boundaries: core::default::Default::default(),
            mv_temporal_predictor: core::default::Default::default(),
            par_denominator: core::default::Default::default(),
            par_numerator: core::default::Default::default(),
            profile: core::default::Default::default(),
            qvbr_quality_level: core::default::Default::default(),
            rate_control_mode: core::default::Default::default(),
            scan_type: core::default::Default::default(),
            scene_change_detect: core::default::Default::default(),
            slices: core::default::Default::default(),
            tier: core::default::Default::default(),
            tile_height: core::default::Default::default(),
            tile_padding: core::default::Default::default(),
            tile_width: core::default::Default::default(),
            timecode_insertion: core::default::Default::default(),
            treeblock_size: core::default::Default::default(),
            color_space_settings: core::default::Default::default(),
            filter_settings: core::default::Default::default(),
            timecode_burnin_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adaptive_quantization` after provisioning.\n"]
    pub fn adaptive_quantization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adaptive_quantization", self.base))
    }

    #[doc = "Get a reference to the value of field `afd_signaling` after provisioning.\n"]
    pub fn afd_signaling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.afd_signaling", self.base))
    }

    #[doc = "Get a reference to the value of field `alternative_transfer_function` after provisioning.\n"]
    pub fn alternative_transfer_function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alternative_transfer_function", self.base))
    }

    #[doc = "Get a reference to the value of field `bitrate` after provisioning.\n"]
    pub fn bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `buf_size` after provisioning.\n"]
    pub fn buf_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buf_size", self.base))
    }

    #[doc = "Get a reference to the value of field `color_metadata` after provisioning.\n"]
    pub fn color_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_afd` after provisioning.\n"]
    pub fn fixed_afd(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_afd", self.base))
    }

    #[doc = "Get a reference to the value of field `flicker_aq` after provisioning.\n"]
    pub fn flicker_aq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flicker_aq", self.base))
    }

    #[doc = "Get a reference to the value of field `framerate_denominator` after provisioning.\n"]
    pub fn framerate_denominator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framerate_denominator", self.base))
    }

    #[doc = "Get a reference to the value of field `framerate_numerator` after provisioning.\n"]
    pub fn framerate_numerator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framerate_numerator", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_closed_cadence` after provisioning.\n"]
    pub fn gop_closed_cadence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_closed_cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_size` after provisioning.\n"]
    pub fn gop_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_size", self.base))
    }

    #[doc = "Get a reference to the value of field `gop_size_units` after provisioning.\n"]
    pub fn gop_size_units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gop_size_units", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc = "Get a reference to the value of field `look_ahead_rate_control` after provisioning.\n"]
    pub fn look_ahead_rate_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.look_ahead_rate_control", self.base))
    }

    #[doc = "Get a reference to the value of field `max_bitrate` after provisioning.\n"]
    pub fn max_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `min_i_interval` after provisioning.\n"]
    pub fn min_i_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_i_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `min_qp` after provisioning.\n"]
    pub fn min_qp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_qp", self.base))
    }

    #[doc = "Get a reference to the value of field `mv_over_picture_boundaries` after provisioning.\n"]
    pub fn mv_over_picture_boundaries(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mv_over_picture_boundaries", self.base))
    }

    #[doc = "Get a reference to the value of field `mv_temporal_predictor` after provisioning.\n"]
    pub fn mv_temporal_predictor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mv_temporal_predictor", self.base))
    }

    #[doc = "Get a reference to the value of field `par_denominator` after provisioning.\n"]
    pub fn par_denominator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.par_denominator", self.base))
    }

    #[doc = "Get a reference to the value of field `par_numerator` after provisioning.\n"]
    pub fn par_numerator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.par_numerator", self.base))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }

    #[doc = "Get a reference to the value of field `qvbr_quality_level` after provisioning.\n"]
    pub fn qvbr_quality_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qvbr_quality_level", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_control_mode` after provisioning.\n"]
    pub fn rate_control_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_control_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scene_change_detect` after provisioning.\n"]
    pub fn scene_change_detect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scene_change_detect", self.base))
    }

    #[doc = "Get a reference to the value of field `slices` after provisioning.\n"]
    pub fn slices(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slices", self.base))
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }

    #[doc = "Get a reference to the value of field `tile_height` after provisioning.\n"]
    pub fn tile_height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tile_height", self.base))
    }

    #[doc = "Get a reference to the value of field `tile_padding` after provisioning.\n"]
    pub fn tile_padding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tile_padding", self.base))
    }

    #[doc = "Get a reference to the value of field `tile_width` after provisioning.\n"]
    pub fn tile_width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tile_width", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_insertion` after provisioning.\n"]
    pub fn timecode_insertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timecode_insertion", self.base))
    }

    #[doc = "Get a reference to the value of field `treeblock_size` after provisioning.\n"]
    pub fn treeblock_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.treeblock_size", self.base))
    }

    #[doc = "Get a reference to the value of field `color_space_settings` after provisioning.\n"]
    pub fn color_space_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_ColorSpaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.color_space_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_settings` after provisioning.\n"]
    pub fn filter_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings_FilterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_burnin_settings` after provisioning.\n"]
    pub fn timecode_burnin_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings_TimecodeBurninSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timecode_burnin_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsDynamic {
    frame_capture_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings>,
    >,
    h264_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings>>,
    h265_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h264_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h265_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings>>,
    dynamic: MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {
    #[doc = "Set the field `frame_capture_settings`.\n"]
    pub fn set_frame_capture_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_capture_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_capture_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `h264_settings`.\n"]
    pub fn set_h264_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.h264_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.h264_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `h265_settings`.\n"]
    pub fn set_h265_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265Settings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.h265_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.h265_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings {
            frame_capture_settings: core::default::Default::default(),
            h264_settings: core::default::Default::default(),
            h265_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frame_capture_settings` after provisioning.\n"]
    pub fn frame_capture_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_FrameCaptureSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `h264_settings` after provisioning.\n"]
    pub fn h264_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H264SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.h264_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `h265_settings` after provisioning.\n"]
    pub fn h265_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings_H265SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.h265_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettings_VideoDescriptionsDynamic {
    codec_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    height: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respond_to_afd: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharpness: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    width: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec_settings: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings>>,
    dynamic: MedialiveChannel_EncoderSettings_VideoDescriptionsDynamic,
}

impl MedialiveChannel_EncoderSettings_VideoDescriptions {
    #[doc = "Set the field `height`.\n"]
    pub fn set_height(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `respond_to_afd`.\n"]
    pub fn set_respond_to_afd(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.respond_to_afd = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_behavior`.\n"]
    pub fn set_scaling_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scaling_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sharpness`.\n"]
    pub fn set_sharpness(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sharpness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `width`.\n"]
    pub fn set_width(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `codec_settings`.\n"]
    pub fn set_codec_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.codec_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.codec_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings_VideoDescriptions {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings_VideoDescriptions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_EncoderSettings_VideoDescriptions {
    pub fn build(self) -> MedialiveChannel_EncoderSettings_VideoDescriptions {
        MedialiveChannel_EncoderSettings_VideoDescriptions {
            height: core::default::Default::default(),
            name: self.name,
            respond_to_afd: core::default::Default::default(),
            scaling_behavior: core::default::Default::default(),
            sharpness: core::default::Default::default(),
            width: core::default::Default::default(),
            codec_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettings_VideoDescriptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettings_VideoDescriptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_EncoderSettings_VideoDescriptionsRef {
        MedialiveChannel_EncoderSettings_VideoDescriptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettings_VideoDescriptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `height` after provisioning.\n"]
    pub fn height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `respond_to_afd` after provisioning.\n"]
    pub fn respond_to_afd(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respond_to_afd", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_behavior` after provisioning.\n"]
    pub fn scaling_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `sharpness` after provisioning.\n"]
    pub fn sharpness(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharpness", self.base))
    }

    #[doc = "Get a reference to the value of field `width` after provisioning.\n"]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }

    #[doc = "Get a reference to the value of field `codec_settings` after provisioning.\n"]
    pub fn codec_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptions_CodecSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.codec_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_EncoderSettingsDynamic {
    audio_descriptions: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AudioDescriptions>>,
    avail_blanking: std::option::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_AvailBlanking>>,
    caption_descriptions: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_CaptionDescriptions>>,
    global_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_GlobalConfiguration>>,
    motion_graphics_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration>>,
    nielsen_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_NielsenConfiguration>>,
    output_groups: std::option::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_OutputGroups>>,
    timecode_config: std::option::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_TimecodeConfig>>,
    video_descriptions: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_EncoderSettings_VideoDescriptions>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_EncoderSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_descriptions: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AudioDescriptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avail_blanking: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_AvailBlanking>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_descriptions: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_CaptionDescriptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_configuration: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_GlobalConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    motion_graphics_configuration: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_configuration: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_NielsenConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_groups: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_OutputGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_config: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_TimecodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_descriptions: std::option::Option<std::vec::Vec<MedialiveChannel_EncoderSettings_VideoDescriptions>>,
    dynamic: MedialiveChannel_EncoderSettingsDynamic,
}

impl MedialiveChannel_EncoderSettings {
    #[doc = "Set the field `audio_descriptions`.\n"]
    pub fn set_audio_descriptions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AudioDescriptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_descriptions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_descriptions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `avail_blanking`.\n"]
    pub fn set_avail_blanking(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_AvailBlanking>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avail_blanking = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avail_blanking = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `caption_descriptions`.\n"]
    pub fn set_caption_descriptions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_CaptionDescriptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.caption_descriptions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.caption_descriptions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `global_configuration`.\n"]
    pub fn set_global_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_GlobalConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.global_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.global_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `motion_graphics_configuration`.\n"]
    pub fn set_motion_graphics_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_MotionGraphicsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.motion_graphics_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.motion_graphics_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nielsen_configuration`.\n"]
    pub fn set_nielsen_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_NielsenConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nielsen_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nielsen_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_groups`.\n"]
    pub fn set_output_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_OutputGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timecode_config`.\n"]
    pub fn set_timecode_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_TimecodeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timecode_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timecode_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_descriptions`.\n"]
    pub fn set_video_descriptions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_EncoderSettings_VideoDescriptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_descriptions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_descriptions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_EncoderSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_EncoderSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_EncoderSettings {}

impl BuildMedialiveChannel_EncoderSettings {
    pub fn build(self) -> MedialiveChannel_EncoderSettings {
        MedialiveChannel_EncoderSettings {
            audio_descriptions: core::default::Default::default(),
            avail_blanking: core::default::Default::default(),
            caption_descriptions: core::default::Default::default(),
            global_configuration: core::default::Default::default(),
            motion_graphics_configuration: core::default::Default::default(),
            nielsen_configuration: core::default::Default::default(),
            output_groups: core::default::Default::default(),
            timecode_config: core::default::Default::default(),
            video_descriptions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_EncoderSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_EncoderSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_EncoderSettingsRef {
        MedialiveChannel_EncoderSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_EncoderSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `avail_blanking` after provisioning.\n"]
    pub fn avail_blanking(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_AvailBlankingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avail_blanking", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_descriptions` after provisioning.\n"]
    pub fn caption_descriptions(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_CaptionDescriptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.caption_descriptions", self.base))
    }

    #[doc = "Get a reference to the value of field `global_configuration` after provisioning.\n"]
    pub fn global_configuration(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_GlobalConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `motion_graphics_configuration` after provisioning.\n"]
    pub fn motion_graphics_configuration(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_EncoderSettings_MotionGraphicsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.motion_graphics_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_configuration` after provisioning.\n"]
    pub fn nielsen_configuration(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_NielsenConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `output_groups` after provisioning.\n"]
    pub fn output_groups(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_OutputGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_config` after provisioning.\n"]
    pub fn timecode_config(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_TimecodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timecode_config", self.base))
    }

    #[doc = "Get a reference to the value of field `video_descriptions` after provisioning.\n"]
    pub fn video_descriptions(&self) -> terrars::ListRef<MedialiveChannel_EncoderSettings_VideoDescriptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_descriptions", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
    audio_selector_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_silence_threshold_msec: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
    #[doc = "Set the field `audio_silence_threshold_msec`.\n"]
    pub fn set_audio_silence_threshold_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audio_silence_threshold_msec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
    #[doc = ""]
    pub audio_selector_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings {
            audio_selector_name: self.audio_selector_name,
            audio_silence_threshold_msec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettingsRef {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_selector_name` after provisioning.\n"]
    pub fn audio_selector_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_selector_name", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_silence_threshold_msec` after provisioning.\n"]
    pub fn audio_silence_threshold_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_silence_threshold_msec", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_threshold_msec: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {
    #[doc = "Set the field `input_loss_threshold_msec`.\n"]
    pub fn set_input_loss_threshold_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.input_loss_threshold_msec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {}

impl BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings {
            input_loss_threshold_msec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettingsRef {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_loss_threshold_msec` after provisioning.\n"]
    pub fn input_loss_threshold_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_threshold_msec", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    black_detect_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_black_threshold_msec: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {
    #[doc = "Set the field `black_detect_threshold`.\n"]
    pub fn set_black_detect_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.black_detect_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_black_threshold_msec`.\n"]
    pub fn set_video_black_threshold_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.video_black_threshold_msec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {}

impl BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings {
            black_detect_threshold: core::default::Default::default(),
            video_black_threshold_msec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettingsRef {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `black_detect_threshold` after provisioning.\n"]
    pub fn black_detect_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.black_detect_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `video_black_threshold_msec` after provisioning.\n"]
    pub fn video_black_threshold_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_black_threshold_msec", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsDynamic {
    audio_silence_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings,
        >,
    >,
    input_loss_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings,
        >,
    >,
    video_black_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_silence_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_black_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings,
        >,
    >,
    dynamic: MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {
    #[doc = "Set the field `audio_silence_settings`.\n"]
    pub fn set_audio_silence_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_silence_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_silence_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_loss_settings`.\n"]
    pub fn set_input_loss_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_loss_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_loss_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_black_settings`.\n"]
    pub fn set_video_black_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_black_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_black_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {}

impl BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings {
            audio_silence_settings: core::default::Default::default(),
            input_loss_settings: core::default::Default::default(),
            video_black_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsRef {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_silence_settings` after provisioning.\n"]
    pub fn audio_silence_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_AudioSilenceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_silence_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_settings` after provisioning.\n"]
    pub fn input_loss_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_InputLossSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_loss_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `video_black_settings` after provisioning.\n"]
    pub fn video_black_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings_VideoBlackSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_black_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionDynamic {
    failover_condition_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_condition_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings,
        >,
    >,
    dynamic: MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionDynamic,
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {
    #[doc = "Set the field `failover_condition_settings`.\n"]
    pub fn set_failover_condition_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failover_condition_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_condition_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {
    type O =
        terrars::BlockAssignable<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {}

impl BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {
    pub fn build(self) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition {
            failover_condition_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionRef {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover_condition_settings` after provisioning.\n"]
    pub fn failover_condition_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition_FailoverConditionSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_condition_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsDynamic {
    failover_condition: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_clear_time_msec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    secondary_input_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_condition: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition>>,
    dynamic: MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
    #[doc = "Set the field `error_clear_time_msec`.\n"]
    pub fn set_error_clear_time_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.error_clear_time_msec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_preference`.\n"]
    pub fn set_input_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failover_condition`.\n"]
    pub fn set_failover_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings_FailoverCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failover_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
    #[doc = ""]
    pub secondary_input_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
    pub fn build(self) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings {
            error_clear_time_msec: core::default::Default::default(),
            input_preference: core::default::Default::default(),
            secondary_input_id: self.secondary_input_id,
            failover_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsRef {
        MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_clear_time_msec` after provisioning.\n"]
    pub fn error_clear_time_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_clear_time_msec", self.base))
    }

    #[doc = "Get a reference to the value of field `input_preference` after provisioning.\n"]
    pub fn input_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_input_id` after provisioning.\n"]
    pub fn secondary_input_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_input_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection {
    group_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection { }

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection {
    #[doc = ""]
    pub group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection {
            group_id: self.group_id,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelectionRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_selection_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
    #[doc = "Set the field `language_selection_policy`.\n"]
    pub fn set_language_selection_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_selection_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection {
            language_code: self.language_code,
            language_selection_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelectionRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_selection_policy` after provisioning.\n"]
    pub fn language_selection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_selection_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection {
    pid: terrars::PrimField<f64>,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection { }

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection {
    #[doc = ""]
    pub pid: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection {
            pid: self.pid,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelectionRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pid` after provisioning.\n"]
    pub fn pid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {
    program_selection: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {

}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {
    #[doc = ""]
    pub program_selection: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode {
            program_selection: self.program_selection,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecodeRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `program_selection` after provisioning.\n"]
    pub fn program_selection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_selection", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks {
    track: terrars::PrimField<f64>,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks { }

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks {
    #[doc = ""]
    pub track: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks {
            track: self.track,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_TracksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_TracksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_TracksRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_TracksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_TracksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `track` after provisioning.\n"]
    pub fn track(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionDynamic {
    dolby_e_decode: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode,
        >,
    >,
    tracks: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dolby_e_decode: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracks: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks,
        >,
    >,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {
    #[doc = "Set the field `dolby_e_decode`.\n"]
    pub fn set_dolby_e_decode(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dolby_e_decode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dolby_e_decode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tracks`.\n"]
    pub fn set_tracks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_Tracks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tracks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tracks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection {
            dolby_e_decode: core::default::Default::default(),
            tracks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dolby_e_decode` after provisioning.\n"]
    pub fn dolby_e_decode(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection_DolbyEDecodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dolby_e_decode", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsDynamic {
    audio_hls_rendition_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection,
        >,
    >,
    audio_language_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection,
        >,
    >,
    audio_pid_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection,
        >,
    >,
    audio_track_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_hls_rendition_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_language_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_pid_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_track_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection>,
    >,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {
    #[doc = "Set the field `audio_hls_rendition_selection`.\n"]
    pub fn set_audio_hls_rendition_selection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_hls_rendition_selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_hls_rendition_selection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `audio_language_selection`.\n"]
    pub fn set_audio_language_selection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_language_selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_language_selection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `audio_pid_selection`.\n"]
    pub fn set_audio_pid_selection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_pid_selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_pid_selection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `audio_track_selection`.\n"]
    pub fn set_audio_track_selection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_track_selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_track_selection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {
    type O =
        terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings {
            audio_hls_rendition_selection: core::default::Default::default(),
            audio_language_selection: core::default::Default::default(),
            audio_pid_selection: core::default::Default::default(),
            audio_track_selection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_hls_rendition_selection` after provisioning.\n"]
    pub fn audio_hls_rendition_selection(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioHlsRenditionSelectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_hls_rendition_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_language_selection` after provisioning.\n"]
    pub fn audio_language_selection(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioLanguageSelectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_language_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_pid_selection` after provisioning.\n"]
    pub fn audio_pid_selection(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioPidSelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_pid_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_track_selection` after provisioning.\n"]
    pub fn audio_track_selection(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings_AudioTrackSelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_track_selection", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_AudioSelectorDynamic {
    selector_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelector {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selector_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings>>,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_AudioSelectorDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelector {
    #[doc = "Set the field `selector_settings`.\n"]
    pub fn set_selector_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selector_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selector_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_AudioSelector {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_AudioSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_AudioSelector {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelector {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelector {
            name: self.name,
            selector_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_AudioSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_AudioSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_AudioSelectorRef {
        MedialiveChannel_InputAttachments_InputSettings_AudioSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_AudioSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `selector_settings` after provisioning.\n"]
    pub fn selector_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_AudioSelector_SelectorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selector_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ancillary_channel_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {
    #[doc = "Set the field `source_ancillary_channel_number`.\n"]
    pub fn set_source_ancillary_channel_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.source_ancillary_channel_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings {
            source_ancillary_channel_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_ancillary_channel_number` after provisioning.\n"]
    pub fn source_ancillary_channel_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ancillary_channel_number", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings {}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings { }

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings {}
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {
    #[doc = "Set the field `ocr_language`.\n"]
    pub fn set_ocr_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocr_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid`.\n"]
    pub fn set_pid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings {
            ocr_language: core::default::Default::default(),
            pid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ocr_language` after provisioning.\n"]
    pub fn ocr_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocr_language", self.base))
    }

    #[doc = "Get a reference to the value of field `pid` after provisioning.\n"]
    pub fn pid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_608_to_708: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte20_detection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_608_channel_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {
    #[doc = "Set the field `convert_608_to_708`.\n"]
    pub fn set_convert_608_to_708(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.convert_608_to_708 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte20_detection`.\n"]
    pub fn set_scte20_detection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte20_detection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_608_channel_number`.\n"]
    pub fn set_source_608_channel_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.source_608_channel_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings {
            convert_608_to_708: core::default::Default::default(),
            scte20_detection: core::default::Default::default(),
            source_608_channel_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `convert_608_to_708` after provisioning.\n"]
    pub fn convert_608_to_708(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.convert_608_to_708", self.base))
    }

    #[doc = "Get a reference to the value of field `scte20_detection` after provisioning.\n"]
    pub fn scte20_detection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte20_detection", self.base))
    }

    #[doc = "Get a reference to the value of field `source_608_channel_number` after provisioning.\n"]
    pub fn source_608_channel_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_608_channel_number", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_608_to_708: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_608_channel_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {
    #[doc = "Set the field `convert_608_to_708`.\n"]
    pub fn set_convert_608_to_708(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.convert_608_to_708 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_608_channel_number`.\n"]
    pub fn set_source_608_channel_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.source_608_channel_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings {
            convert_608_to_708: core::default::Default::default(),
            source_608_channel_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `convert_608_to_708` after provisioning.\n"]
    pub fn convert_608_to_708(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.convert_608_to_708", self.base))
    }

    #[doc = "Get a reference to the value of field `source_608_channel_number` after provisioning.\n"]
    pub fn source_608_channel_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_608_channel_number", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {
    #[doc = "Set the field `ocr_language`.\n"]
    pub fn set_ocr_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocr_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid`.\n"]
    pub fn set_pid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings {
            ocr_language: core::default::Default::default(),
            pid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ocr_language` after provisioning.\n"]
    pub fn ocr_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocr_language", self.base))
    }

    #[doc = "Get a reference to the value of field `pid` after provisioning.\n"]
    pub fn pid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {
    height: terrars::PrimField<f64>,
    left_offset: terrars::PrimField<f64>,
    top_offset: terrars::PrimField<f64>,
    width: terrars::PrimField<f64>,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {

}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {
    #[doc = ""]
    pub height: terrars::PrimField<f64>,
    #[doc = ""]
    pub left_offset: terrars::PrimField<f64>,
    #[doc = ""]
    pub top_offset: terrars::PrimField<f64>,
    #[doc = ""]
    pub width: terrars::PrimField<f64>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle {
            height: self.height,
            left_offset: self.left_offset,
            top_offset: self.top_offset,
            width: self.width,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangleRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `height` after provisioning.\n"]
    pub fn height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc = "Get a reference to the value of field `left_offset` after provisioning.\n"]
    pub fn left_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.left_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `top_offset` after provisioning.\n"]
    pub fn top_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `width` after provisioning.\n"]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsDynamic {
    output_rectangle: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_rectangle: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle,
        >,
    >,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {
    #[doc = "Set the field `page_number`.\n"]
    pub fn set_page_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.page_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_rectangle`.\n"]
    pub fn set_output_rectangle(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangle,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_rectangle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_rectangle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {
    pub fn build(
        self,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings {
            page_number: core::default::Default::default(),
            output_rectangle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `page_number` after provisioning.\n"]
    pub fn page_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_number", self.base))
    }

    #[doc = "Get a reference to the value of field `output_rectangle` after provisioning.\n"]
    pub fn output_rectangle(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings_OutputRectangleRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_rectangle", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsDynamic {
    ancillary_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings,
        >,
    >,
    arib_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings,
        >,
    >,
    dvb_sub_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings,
        >,
    >,
    embedded_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings,
        >,
    >,
    scte20_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings,
        >,
    >,
    scte27_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings,
        >,
    >,
    teletext_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ancillary_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arib_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_sub_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedded_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte20_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte27_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teletext_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings>,
    >,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {
    #[doc = "Set the field `ancillary_source_settings`.\n"]
    pub fn set_ancillary_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ancillary_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ancillary_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `arib_source_settings`.\n"]
    pub fn set_arib_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.arib_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.arib_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dvb_sub_source_settings`.\n"]
    pub fn set_dvb_sub_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dvb_sub_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dvb_sub_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `embedded_source_settings`.\n"]
    pub fn set_embedded_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.embedded_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.embedded_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scte20_source_settings`.\n"]
    pub fn set_scte20_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scte20_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scte20_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scte27_source_settings`.\n"]
    pub fn set_scte27_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scte27_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scte27_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `teletext_source_settings`.\n"]
    pub fn set_teletext_source_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.teletext_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.teletext_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {
    type O =
        terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings {
            ancillary_source_settings: core::default::Default::default(),
            arib_source_settings: core::default::Default::default(),
            dvb_sub_source_settings: core::default::Default::default(),
            embedded_source_settings: core::default::Default::default(),
            scte20_source_settings: core::default::Default::default(),
            scte27_source_settings: core::default::Default::default(),
            teletext_source_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ancillary_source_settings` after provisioning.\n"]
    pub fn ancillary_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AncillarySourceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ancillary_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `arib_source_settings` after provisioning.\n"]
    pub fn arib_source_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_AribSourceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arib_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_sub_source_settings` after provisioning.\n"]
    pub fn dvb_sub_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_DvbSubSourceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_sub_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `embedded_source_settings` after provisioning.\n"]
    pub fn embedded_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_EmbeddedSourceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.embedded_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `scte20_source_settings` after provisioning.\n"]
    pub fn scte20_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte20SourceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scte20_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `scte27_source_settings` after provisioning.\n"]
    pub fn scte27_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_Scte27SourceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scte27_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `teletext_source_settings` after provisioning.\n"]
    pub fn teletext_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings_TeletextSourceSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teletext_source_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorDynamic {
    selector_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selector_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings>>,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selector_settings`.\n"]
    pub fn set_selector_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selector_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selector_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelector {
            language_code: core::default::Default::default(),
            name: self.name,
            selector_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorRef {
        MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `selector_settings` after provisioning.\n"]
    pub fn selector_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector_SelectorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selector_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bandwidth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_segments: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte35_source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {
    #[doc = "Set the field `bandwidth`.\n"]
    pub fn set_bandwidth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bandwidth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_segments`.\n"]
    pub fn set_buffer_segments(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_segments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retries`.\n"]
    pub fn set_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_interval`.\n"]
    pub fn set_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte35_source`.\n"]
    pub fn set_scte35_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scte35_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {
        MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings {
            bandwidth: core::default::Default::default(),
            buffer_segments: core::default::Default::default(),
            retries: core::default::Default::default(),
            retry_interval: core::default::Default::default(),
            scte35_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_segments` after provisioning.\n"]
    pub fn buffer_segments(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_segments", self.base))
    }

    #[doc = "Get a reference to the value of field `retries` after provisioning.\n"]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_interval` after provisioning.\n"]
    pub fn retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `scte35_source` after provisioning.\n"]
    pub fn scte35_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte35_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsDynamic {
    hls_input_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_validation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_input_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings>>,
    dynamic: MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {
    #[doc = "Set the field `server_validation`.\n"]
    pub fn set_server_validation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hls_input_settings`.\n"]
    pub fn set_hls_input_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hls_input_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hls_input_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {
        MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings {
            server_validation: core::default::Default::default(),
            hls_input_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsRef {
        MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_validation` after provisioning.\n"]
    pub fn server_validation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_input_settings` after provisioning.\n"]
    pub fn hls_input_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings_HlsInputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_input_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings_VideoSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_space: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_space_usage: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_InputAttachments_InputSettings_VideoSelector {
    #[doc = "Set the field `color_space`.\n"]
    pub fn set_color_space(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color_space = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color_space_usage`.\n"]
    pub fn set_color_space_usage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color_space_usage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings_VideoSelector {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_VideoSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings_VideoSelector {}

impl BuildMedialiveChannel_InputAttachments_InputSettings_VideoSelector {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings_VideoSelector {
        MedialiveChannel_InputAttachments_InputSettings_VideoSelector {
            color_space: core::default::Default::default(),
            color_space_usage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettings_VideoSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettings_VideoSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettings_VideoSelectorRef {
        MedialiveChannel_InputAttachments_InputSettings_VideoSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettings_VideoSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color_space` after provisioning.\n"]
    pub fn color_space(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color_space", self.base))
    }

    #[doc = "Get a reference to the value of field `color_space_usage` after provisioning.\n"]
    pub fn color_space_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color_space_usage", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachments_InputSettingsDynamic {
    audio_selector: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_AudioSelector>>,
    caption_selector: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector>>,
    network_input_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings>>,
    video_selector: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings_VideoSelector>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments_InputSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deblock_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denoise_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_strength: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte35_pid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smpte2038_data_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_end_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_selector: std::option::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_AudioSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_selector: std::option::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_input_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_selector: std::option::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings_VideoSelector>>,
    dynamic: MedialiveChannel_InputAttachments_InputSettingsDynamic,
}

impl MedialiveChannel_InputAttachments_InputSettings {
    #[doc = "Set the field `deblock_filter`.\n"]
    pub fn set_deblock_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deblock_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `denoise_filter`.\n"]
    pub fn set_denoise_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.denoise_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_strength`.\n"]
    pub fn set_filter_strength(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.filter_strength = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_filter`.\n"]
    pub fn set_input_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scte35_pid`.\n"]
    pub fn set_scte35_pid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scte35_pid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smpte2038_data_preference`.\n"]
    pub fn set_smpte2038_data_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.smpte2038_data_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_end_behavior`.\n"]
    pub fn set_source_end_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_end_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_selector`.\n"]
    pub fn set_audio_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_AudioSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `caption_selector`.\n"]
    pub fn set_caption_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_CaptionSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.caption_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.caption_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_input_settings`.\n"]
    pub fn set_network_input_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_input_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_input_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_selector`.\n"]
    pub fn set_video_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings_VideoSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments_InputSettings {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments_InputSettings {}

impl BuildMedialiveChannel_InputAttachments_InputSettings {
    pub fn build(self) -> MedialiveChannel_InputAttachments_InputSettings {
        MedialiveChannel_InputAttachments_InputSettings {
            deblock_filter: core::default::Default::default(),
            denoise_filter: core::default::Default::default(),
            filter_strength: core::default::Default::default(),
            input_filter: core::default::Default::default(),
            scte35_pid: core::default::Default::default(),
            smpte2038_data_preference: core::default::Default::default(),
            source_end_behavior: core::default::Default::default(),
            audio_selector: core::default::Default::default(),
            caption_selector: core::default::Default::default(),
            network_input_settings: core::default::Default::default(),
            video_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachments_InputSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachments_InputSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannel_InputAttachments_InputSettingsRef {
        MedialiveChannel_InputAttachments_InputSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachments_InputSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deblock_filter` after provisioning.\n"]
    pub fn deblock_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deblock_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `denoise_filter` after provisioning.\n"]
    pub fn denoise_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.denoise_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_strength` after provisioning.\n"]
    pub fn filter_strength(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_strength", self.base))
    }

    #[doc = "Get a reference to the value of field `input_filter` after provisioning.\n"]
    pub fn input_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `scte35_pid` after provisioning.\n"]
    pub fn scte35_pid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scte35_pid", self.base))
    }

    #[doc = "Get a reference to the value of field `smpte2038_data_preference` after provisioning.\n"]
    pub fn smpte2038_data_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smpte2038_data_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `source_end_behavior` after provisioning.\n"]
    pub fn source_end_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_end_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_selector` after provisioning.\n"]
    pub fn audio_selector(&self) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_AudioSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_selector` after provisioning.\n"]
    pub fn caption_selector(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_CaptionSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.caption_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `network_input_settings` after provisioning.\n"]
    pub fn network_input_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_NetworkInputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_input_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `video_selector` after provisioning.\n"]
    pub fn video_selector(&self) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettings_VideoSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_selector", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannel_InputAttachmentsDynamic {
    automatic_input_failover_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings>>,
    input_settings: std::option::Option<terrars::DynamicBlock<MedialiveChannel_InputAttachments_InputSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputAttachments {
    input_attachment_name: terrars::PrimField<std::string::String>,
    input_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_input_failover_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_settings: std::option::Option<std::vec::Vec<MedialiveChannel_InputAttachments_InputSettings>>,
    dynamic: MedialiveChannel_InputAttachmentsDynamic,
}

impl MedialiveChannel_InputAttachments {
    #[doc = "Set the field `automatic_input_failover_settings`.\n"]
    pub fn set_automatic_input_failover_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.automatic_input_failover_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.automatic_input_failover_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_settings`.\n"]
    pub fn set_input_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannel_InputAttachments_InputSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_InputAttachments {
    type O = terrars::BlockAssignable<MedialiveChannel_InputAttachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputAttachments {
    #[doc = ""]
    pub input_attachment_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub input_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputAttachments {
    pub fn build(self) -> MedialiveChannel_InputAttachments {
        MedialiveChannel_InputAttachments {
            input_attachment_name: self.input_attachment_name,
            input_id: self.input_id,
            automatic_input_failover_settings: core::default::Default::default(),
            input_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputAttachmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_InputAttachmentsRef {
        MedialiveChannel_InputAttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputAttachmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_attachment_name` after provisioning.\n"]
    pub fn input_attachment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_attachment_name", self.base))
    }

    #[doc = "Get a reference to the value of field `input_id` after provisioning.\n"]
    pub fn input_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_id", self.base))
    }

    #[doc = "Get a reference to the value of field `automatic_input_failover_settings` after provisioning.\n"]
    pub fn automatic_input_failover_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannel_InputAttachments_AutomaticInputFailoverSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_input_failover_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `input_settings` after provisioning.\n"]
    pub fn input_settings(&self) -> terrars::ListRef<MedialiveChannel_InputAttachments_InputSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_InputSpecification {
    codec: terrars::PrimField<std::string::String>,
    input_resolution: terrars::PrimField<std::string::String>,
    maximum_bitrate: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_InputSpecification { }

impl terrars::ToListMappable for MedialiveChannel_InputSpecification {
    type O = terrars::BlockAssignable<MedialiveChannel_InputSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_InputSpecification {
    #[doc = ""]
    pub codec: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub input_resolution: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub maximum_bitrate: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_InputSpecification {
    pub fn build(self) -> MedialiveChannel_InputSpecification {
        MedialiveChannel_InputSpecification {
            codec: self.codec,
            input_resolution: self.input_resolution,
            maximum_bitrate: self.maximum_bitrate,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_InputSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_InputSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_InputSpecificationRef {
        MedialiveChannel_InputSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_InputSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `codec` after provisioning.\n"]
    pub fn codec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }

    #[doc = "Get a reference to the value of field `input_resolution` after provisioning.\n"]
    pub fn input_resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_bitrate` after provisioning.\n"]
    pub fn maximum_bitrate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_bitrate", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_Maintenance {
    maintenance_day: terrars::PrimField<std::string::String>,
    maintenance_start_time: terrars::PrimField<std::string::String>,
}

impl MedialiveChannel_Maintenance { }

impl terrars::ToListMappable for MedialiveChannel_Maintenance {
    type O = terrars::BlockAssignable<MedialiveChannel_Maintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Maintenance {
    #[doc = ""]
    pub maintenance_day: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub maintenance_start_time: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannel_Maintenance {
    pub fn build(self) -> MedialiveChannel_Maintenance {
        MedialiveChannel_Maintenance {
            maintenance_day: self.maintenance_day,
            maintenance_start_time: self.maintenance_start_time,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_MaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_MaintenanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_MaintenanceRef {
        MedialiveChannel_MaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_MaintenanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_day` after provisioning.\n"]
    pub fn maintenance_day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_day", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_start_time` after provisioning.\n"]
    pub fn maintenance_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannel_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_Timeouts {
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

impl terrars::ToListMappable for MedialiveChannel_Timeouts {
    type O = terrars::BlockAssignable<MedialiveChannel_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Timeouts {}

impl BuildMedialiveChannel_Timeouts {
    pub fn build(self) -> MedialiveChannel_Timeouts {
        MedialiveChannel_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_TimeoutsRef {
        MedialiveChannel_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_TimeoutsRef {
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

#[derive(Serialize)]
pub struct MedialiveChannel_Vpc {
    public_address_allocation_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannel_Vpc {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannel_Vpc {
    type O = terrars::BlockAssignable<MedialiveChannel_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannel_Vpc {
    #[doc = ""]
    pub public_address_allocation_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMedialiveChannel_Vpc {
    pub fn build(self) -> MedialiveChannel_Vpc {
        MedialiveChannel_Vpc {
            public_address_allocation_ids: self.public_address_allocation_ids,
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveChannel_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannel_VpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannel_VpcRef {
        MedialiveChannel_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannel_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `public_address_allocation_ids` after provisioning.\n"]
    pub fn public_address_allocation_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_address_allocation_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
