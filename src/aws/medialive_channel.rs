use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MedialiveChannelData {
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
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_channel: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdi_input_specification: std::option::Option<std::vec::Vec<MedialiveChannelCdiInputSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<MedialiveChannelDestinationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoder_settings: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_attachments: std::option::Option<std::vec::Vec<MedialiveChannelInputAttachmentsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_specification: std::option::Option<std::vec::Vec<MedialiveChannelInputSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance: std::option::Option<std::vec::Vec<MedialiveChannelMaintenanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MedialiveChannelTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<MedialiveChannelVpcEl>>,
    dynamic: MedialiveChannelDynamic,
}

struct MedialiveChannel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MedialiveChannelData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelCdiInputSpecificationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelDestinationsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelInputAttachmentsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelInputSpecificationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelMaintenanceEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<MedialiveChannelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelVpcEl>>) -> Self {
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
    pub fn cdi_input_specification(&self) -> terrars::ListRef<MedialiveChannelCdiInputSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdi_input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoder_settings` after provisioning.\n"]
    pub fn encoder_settings(&self) -> terrars::ListRef<MedialiveChannelEncoderSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encoder_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_specification` after provisioning.\n"]
    pub fn input_specification(&self) -> terrars::ListRef<MedialiveChannelInputSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> terrars::ListRef<MedialiveChannelMaintenanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveChannelTimeoutsElRef {
        MedialiveChannelTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MedialiveChannelVpcElRef> {
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
            data: std::cell::RefCell::new(MedialiveChannelData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                channel_class: self.channel_class,
                id: core::default::Default::default(),
                log_level: core::default::Default::default(),
                name: self.name,
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
    pub fn cdi_input_specification(&self) -> terrars::ListRef<MedialiveChannelCdiInputSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdi_input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoder_settings` after provisioning.\n"]
    pub fn encoder_settings(&self) -> terrars::ListRef<MedialiveChannelEncoderSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encoder_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_specification` after provisioning.\n"]
    pub fn input_specification(&self) -> terrars::ListRef<MedialiveChannelInputSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> terrars::ListRef<MedialiveChannelMaintenanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveChannelTimeoutsElRef {
        MedialiveChannelTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MedialiveChannelVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelCdiInputSpecificationEl {
    resolution: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelCdiInputSpecificationEl { }

impl terrars::ToListMappable for MedialiveChannelCdiInputSpecificationEl {
    type O = terrars::BlockAssignable<MedialiveChannelCdiInputSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelCdiInputSpecificationEl {
    #[doc = ""]
    pub resolution: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelCdiInputSpecificationEl {
    pub fn build(self) -> MedialiveChannelCdiInputSpecificationEl {
        MedialiveChannelCdiInputSpecificationEl { resolution: self.resolution }
    }
}

pub struct MedialiveChannelCdiInputSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelCdiInputSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelCdiInputSpecificationElRef {
        MedialiveChannelCdiInputSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelCdiInputSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelDestinationsElMediaPackageSettingsEl {
    channel_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelDestinationsElMediaPackageSettingsEl { }

impl terrars::ToListMappable for MedialiveChannelDestinationsElMediaPackageSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelDestinationsElMediaPackageSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelDestinationsElMediaPackageSettingsEl {
    #[doc = ""]
    pub channel_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelDestinationsElMediaPackageSettingsEl {
    pub fn build(self) -> MedialiveChannelDestinationsElMediaPackageSettingsEl {
        MedialiveChannelDestinationsElMediaPackageSettingsEl { channel_id: self.channel_id }
    }
}

pub struct MedialiveChannelDestinationsElMediaPackageSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelDestinationsElMediaPackageSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelDestinationsElMediaPackageSettingsElRef {
        MedialiveChannelDestinationsElMediaPackageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelDestinationsElMediaPackageSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_id` after provisioning.\n"]
    pub fn channel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelDestinationsElMultiplexSettingsEl {
    multiplex_id: terrars::PrimField<std::string::String>,
    program_name: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelDestinationsElMultiplexSettingsEl { }

impl terrars::ToListMappable for MedialiveChannelDestinationsElMultiplexSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelDestinationsElMultiplexSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelDestinationsElMultiplexSettingsEl {
    #[doc = ""]
    pub multiplex_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub program_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelDestinationsElMultiplexSettingsEl {
    pub fn build(self) -> MedialiveChannelDestinationsElMultiplexSettingsEl {
        MedialiveChannelDestinationsElMultiplexSettingsEl {
            multiplex_id: self.multiplex_id,
            program_name: self.program_name,
        }
    }
}

pub struct MedialiveChannelDestinationsElMultiplexSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelDestinationsElMultiplexSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelDestinationsElMultiplexSettingsElRef {
        MedialiveChannelDestinationsElMultiplexSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelDestinationsElMultiplexSettingsElRef {
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
pub struct MedialiveChannelDestinationsElSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelDestinationsElSettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelDestinationsElSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelDestinationsElSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelDestinationsElSettingsEl {}

impl BuildMedialiveChannelDestinationsElSettingsEl {
    pub fn build(self) -> MedialiveChannelDestinationsElSettingsEl {
        MedialiveChannelDestinationsElSettingsEl {
            password_param: core::default::Default::default(),
            stream_name: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelDestinationsElSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelDestinationsElSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelDestinationsElSettingsElRef {
        MedialiveChannelDestinationsElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelDestinationsElSettingsElRef {
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
struct MedialiveChannelDestinationsElDynamic {
    media_package_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelDestinationsElMediaPackageSettingsEl>>,
    multiplex_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelDestinationsElMultiplexSettingsEl>>,
    settings: std::option::Option<terrars::DynamicBlock<MedialiveChannelDestinationsElSettingsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelDestinationsEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_package_settings: std::option::Option<std::vec::Vec<MedialiveChannelDestinationsElMediaPackageSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_settings: std::option::Option<std::vec::Vec<MedialiveChannelDestinationsElMultiplexSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<MedialiveChannelDestinationsElSettingsEl>>,
    dynamic: MedialiveChannelDestinationsElDynamic,
}

impl MedialiveChannelDestinationsEl {
    #[doc = "Set the field `media_package_settings`.\n"]
    pub fn set_media_package_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelDestinationsElMediaPackageSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelDestinationsElMultiplexSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelDestinationsElSettingsEl>>,
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

impl terrars::ToListMappable for MedialiveChannelDestinationsEl {
    type O = terrars::BlockAssignable<MedialiveChannelDestinationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelDestinationsEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelDestinationsEl {
    pub fn build(self) -> MedialiveChannelDestinationsEl {
        MedialiveChannelDestinationsEl {
            id: self.id,
            media_package_settings: core::default::Default::default(),
            multiplex_settings: core::default::Default::default(),
            settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelDestinationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelDestinationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelDestinationsElRef {
        MedialiveChannelDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelDestinationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `multiplex_settings` after provisioning.\n"]
    pub fn multiplex_settings(&self) -> terrars::ListRef<MedialiveChannelDestinationsElMultiplexSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_lkfs: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl {
            algorithm: core::default::Default::default(),
            algorithm_control: core::default::Default::default(),
            target_lkfs: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {
    cbet_check_digit_string: terrars::PrimField<std::string::String>,
    cbet_stepaside: terrars::PrimField<std::string::String>,
    csid: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {

}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {
    #[doc = ""]
    pub cbet_check_digit_string: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub cbet_stepaside: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub csid: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl {
            cbet_check_digit_string: self.cbet_check_digit_string,
            cbet_stepaside: self.cbet_stepaside,
            csid: self.csid,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {
    check_digit_string: terrars::PrimField<std::string::String>,
    sid: terrars::PrimField<f64>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {

}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {
    #[doc = ""]
    pub check_digit_string: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sid: terrars::PrimField<f64>,
}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl {
            check_digit_string: self.check_digit_string,
            sid: self.sid,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElDynamic {
    nielsen_cbet_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl,
        >,
    >,
    nielsen_naes_ii_nw_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_cbet_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_naes_ii_nw_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl {
            nielsen_distribution_type: core::default::Default::default(),
            nielsen_cbet_settings: core::default::Default::default(),
            nielsen_naes_ii_nw_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef {
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
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenCbetSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_cbet_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `nielsen_naes_ii_nw_settings` after provisioning.\n"]
    pub fn nielsen_naes_ii_nw_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElNielsenNaesIiNwSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_naes_ii_nw_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElDynamic {
    nielsen_watermarks_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_watermarks_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl>,
    >,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl {
            nielsen_watermarks_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nielsen_watermarks_settings` after provisioning.\n"]
    pub fn nielsen_watermarks_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElNielsenWatermarksSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nielsen_watermarks_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vbr_quality: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            input_type: core::default::Default::default(),
            profile: core::default::Default::default(),
            raw_format: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
            spec: core::default::Default::default(),
            vbr_quality: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
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

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
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

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
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

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
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

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl {
            bitrate: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl {}
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bit_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coding_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl {
            bit_depth: core::default::Default::default(),
            coding_mode: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElDynamic {
    aac_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl>>,
    ac3_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl>>,
    eac3_atmos_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl>,
    >,
    eac3_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl>,
    >,
    mp2_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl>>,
    pass_through_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl>,
    >,
    wav_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aac_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ac3_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eac3_atmos_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eac3_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mp2_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pass_through_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wav_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsEl,
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aac_settings` after provisioning.\n"]
    pub fn aac_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAacSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aac_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ac3_settings` after provisioning.\n"]
    pub fn ac3_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElAc3SettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ac3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `eac3_atmos_settings` after provisioning.\n"]
    pub fn eac3_atmos_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3AtmosSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eac3_atmos_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `eac3_settings` after provisioning.\n"]
    pub fn eac3_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElEac3SettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eac3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `mp2_settings` after provisioning.\n"]
    pub fn mp2_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElMp2SettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mp2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pass_through_settings` after provisioning.\n"]
    pub fn pass_through_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElPassThroughSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pass_through_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `wav_settings` after provisioning.\n"]
    pub fn wav_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElWavSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wav_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl {
    gain: terrars::PrimField<f64>,
    input_channel: terrars::PrimField<f64>,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl {
    #[doc = ""]
    pub gain: terrars::PrimField<f64>,
    #[doc = ""]
    pub input_channel: terrars::PrimField<f64>,
}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl {
            gain: self.gain,
            input_channel: self.input_channel,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsElRef {
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
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElDynamic {
    input_channel_levels: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
    output_channel: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_channel_levels: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElInputChannelLevelsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
    #[doc = ""]
    pub output_channel: terrars::PrimField<f64>,
}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl {
            output_channel: self.output_channel,
            input_channel_levels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_channel` after provisioning.\n"]
    pub fn output_channel(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_channel", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElDynamic {
    channel_mappings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channels_in: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channels_out: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_mappings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElChannelMappingsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl {
            channels_in: core::default::Default::default(),
            channels_out: core::default::Default::default(),
            channel_mappings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElAudioDescriptionsElDynamic {
    audio_normalization_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl>>,
    audio_watermark_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl>>,
    codec_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>>,
    remix_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
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
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_watermark_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remix_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElAudioDescriptionsElDynamic,
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
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
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsEl,
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
                        ::BlockAssignable<
                            MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsEl,
                        >,
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
                        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsEl>,
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
                        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsEl>,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    #[doc = ""]
    pub audio_selector_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElAudioDescriptionsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
        MedialiveChannelEncoderSettingsElAudioDescriptionsEl {
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

pub struct MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
        MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAudioDescriptionsElRef {
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
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioNormalizationSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_normalization_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_watermark_settings` after provisioning.\n"]
    pub fn audio_watermark_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElAudioWatermarkSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_watermark_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `codec_settings` after provisioning.\n"]
    pub fn codec_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElCodecSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.codec_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `remix_settings` after provisioning.\n"]
    pub fn remix_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAudioDescriptionsElRemixSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remix_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
        MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
        MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef {
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
struct MedialiveChannelEncoderSettingsElAvailBlankingElDynamic {
    avail_blanking_image: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElAvailBlankingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avail_blanking_image: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>>,
    dynamic: MedialiveChannelEncoderSettingsElAvailBlankingElDynamic,
}

impl MedialiveChannelEncoderSettingsElAvailBlankingEl {
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
                        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageEl>,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElAvailBlankingEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElAvailBlankingEl {}

impl BuildMedialiveChannelEncoderSettingsElAvailBlankingEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElAvailBlankingEl {
        MedialiveChannelEncoderSettingsElAvailBlankingEl {
            state: core::default::Default::default(),
            avail_blanking_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElAvailBlankingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElAvailBlankingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElAvailBlankingElRef {
        MedialiveChannelEncoderSettingsElAvailBlankingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElAvailBlankingElRef {
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
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAvailBlankingElAvailBlankingImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avail_blanking_image", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl {
            canned_acl: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElDynamic {
    archive_s3_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_s3_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl {
            archive_s3_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_s3_settings` after provisioning.\n"]
    pub fn archive_s3_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElArchiveS3SettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_s3_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDynamic {
    archive_cdn_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl,
        >,
    >,
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollover_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_cdn_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl>,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl {
            rollover_interval: core::default::Default::default(),
            archive_cdn_settings: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElArchiveCdnSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_cdn_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl {
            canned_acl: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElDynamic {
    frame_capture_s3_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_s3_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl {
            frame_capture_s3_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frame_capture_s3_settings` after provisioning.\n"]
    pub fn frame_capture_s3_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElFrameCaptureS3SettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_s3_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl,
        >,
    >,
    frame_capture_cdn_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_cdn_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl {
            destination: core::default::Default::default(),
            frame_capture_cdn_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_cdn_settings` after provisioning.\n"]
    pub fn frame_capture_cdn_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElFrameCaptureCdnSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_cdn_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {
    caption_channel: terrars::PrimField<f64>,
    language_code: terrars::PrimField<std::string::String>,
    language_description: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {

}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {
    #[doc = ""]
    pub caption_channel: terrars::PrimField<f64>,
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub language_description: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl {
            caption_channel: self.caption_channel,
            language_code: self.language_code,
            language_description: self.language_description,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {
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

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filecache_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_delay: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {
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

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            media_store_storage_class: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.canned_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl {
            canned_acl: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {
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

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl {
            connection_retry_interval: core::default::Default::default(),
            filecache_duration: core::default::Default::default(),
            http_transfer_mode: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            restart_delay: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElDynamic {
    hls_akamai_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl,
        >,
    >,
    hls_basic_put_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl,
        >,
    >,
    hls_media_store_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl,
        >,
    >,
    hls_s3_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl,
        >,
    >,
    hls_webdav_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_akamai_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_basic_put_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_media_store_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_s3_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_webdav_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl {
            hls_akamai_settings: core::default::Default::default(),
            hls_basic_put_settings: core::default::Default::default(),
            hls_media_store_settings: core::default::Default::default(),
            hls_s3_settings: core::default::Default::default(),
            hls_webdav_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hls_akamai_settings` after provisioning.\n"]
    pub fn hls_akamai_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsAkamaiSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_akamai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_basic_put_settings` after provisioning.\n"]
    pub fn hls_basic_put_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsBasicPutSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_basic_put_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_media_store_settings` after provisioning.\n"]
    pub fn hls_media_store_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsMediaStoreSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_media_store_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_s3_settings` after provisioning.\n"]
    pub fn hls_s3_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsS3SettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_s3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_webdav_settings` after provisioning.\n"]
    pub fn hls_webdav_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElHlsWebdavSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_webdav_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef {
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
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElDynamic {
    key_provider_server: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
    static_key_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_provider_server: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
    #[doc = ""]
    pub static_key_value: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl {
            static_key_value: self.static_key_value,
            key_provider_server: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElKeyProviderServerElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_provider_server", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElDynamic {
    static_key_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_key_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl {
            static_key_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_key_settings` after provisioning.\n"]
    pub fn static_key_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElStaticKeySettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_key_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDynamic {
    caption_language_mappings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl,
        >,
    >,
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl,
        >,
    >,
    hls_cdn_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl,
        >,
    >,
    key_provider_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
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
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_cdn_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_provider_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElCaptionLanguageMappingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsEl,
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

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef {
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
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_cdn_settings` after provisioning.\n"]
    pub fn hls_cdn_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElHlsCdnSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_cdn_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `key_provider_settings` after provisioning.\n"]
    pub fn key_provider_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElKeyProviderSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_provider_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl {
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acquisition_point_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_only_timecodec_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_id: std::option::Option<terrars::PrimField<f64>>,
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
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
    #[doc = "Set the field `acquisition_point_id`.\n"]
    pub fn set_acquisition_point_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.acquisition_point_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_only_timecodec_control`.\n"]
    pub fn set_audio_only_timecodec_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_only_timecodec_control = std::option::Option::Some(v.into());
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
    pub fn set_event_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl {
            acquisition_point_id: core::default::Default::default(),
            audio_only_timecodec_control: core::default::Default::default(),
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acquisition_point_id` after provisioning.\n"]
    pub fn acquisition_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acquisition_point_id", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_only_timecodec_control` after provisioning.\n"]
    pub fn audio_only_timecodec_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_only_timecodec_control", self.base))
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
    pub fn event_id(&self) -> terrars::PrimExpr<f64> {
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
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl {}
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
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

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
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

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_id3_frame: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_id3_period: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
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

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl {
            input_loss_action: core::default::Default::default(),
            timed_metadata_id3_frame: core::default::Default::default(),
            timed_metadata_id3_period: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElDynamic {
    archive_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl>,
    >,
    frame_capture_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl,
        >,
    >,
    hls_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl>,
    >,
    media_package_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl,
        >,
    >,
    ms_smooth_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl>,
    >,
    multiplex_group_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl>,
    >,
    rtmp_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl>,
    >,
    udp_group_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_package_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ms_smooth_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_group_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rtmp_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_group_settings` after provisioning.\n"]
    pub fn archive_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElArchiveGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_group_settings` after provisioning.\n"]
    pub fn frame_capture_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElFrameCaptureGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_group_settings` after provisioning.\n"]
    pub fn hls_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElHlsGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `media_package_group_settings` after provisioning.\n"]
    pub fn media_package_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMediaPackageGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_package_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ms_smooth_group_settings` after provisioning.\n"]
    pub fn ms_smooth_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMsSmoothGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ms_smooth_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `multiplex_group_settings` after provisioning.\n"]
    pub fn multiplex_group_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElMultiplexGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rtmp_group_settings` after provisioning.\n"]
    pub fn rtmp_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRtmpGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rtmp_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_group_settings` after provisioning.\n"]
    pub fn udp_group_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElUdpGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.udp_group_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    network_id: terrars::PrimField<f64>,
    network_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    #[doc = ""]
    pub network_id: terrars::PrimField<f64>,
    #[doc = ""]
    pub network_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
            network_id: self.network_id,
            network_name: self.network_name,
            rep_interval: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_sdt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_provider_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
            output_sdt: core::default::Default::default(),
            rep_interval: core::default::Default::default(),
            service_name: core::default::Default::default(),
            service_provider_name: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
            rep_interval: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"]
    pub fn rep_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rep_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDynamic {
    dvb_nit_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
        >,
    >,
    dvb_sdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
        >,
    >,
    dvb_tdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {
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
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_sdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_tdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_nit_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_sdt_settings` after provisioning.\n"]
    pub fn dvb_sdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_sdt_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_tdt_settings` after provisioning.\n"]
    pub fn dvb_tdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_tdt_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {

}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl {}
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElDynamic {
    m2ts_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl,
        >,
    >,
    raw_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    m2ts_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl {
            m2ts_settings: core::default::Default::default(),
            raw_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `m2ts_settings` after provisioning.\n"]
    pub fn m2ts_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElM2tsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.m2ts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_settings` after provisioning.\n"]
    pub fn raw_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRawSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElDynamic {
    container_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {
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
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl {
            extension: core::default::Default::default(),
            name_modifier: core::default::Default::default(),
            container_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElContainerSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {
    #[doc = "Set the field `name_modifier`.\n"]
    pub fn set_name_modifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_modifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl {
            name_modifier: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name_modifier` after provisioning.\n"]
    pub fn name_modifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_modifier", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl {
            password_param: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef {
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
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElDynamic {
    audio_only_image: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {
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
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl {
            audio_group_id: core::default::Default::default(),
            audio_track_type: core::default::Default::default(),
            segment_type: core::default::Default::default(),
            audio_only_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElAudioOnlyImageElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_only_image", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_rendition_sets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nielsen_id3_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_metadata_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl {
            audio_rendition_sets: core::default::Default::default(),
            nielsen_id3_behavior: core::default::Default::default(),
            timed_metadata_behavior: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {

}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl {}
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {
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

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef {
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
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElDynamic {
    m3u8_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_rendition_sets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    m3u8_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl {
            audio_rendition_sets: core::default::Default::default(),
            m3u8_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElM3u8SettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.m3u8_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElDynamic {
    audio_only_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl,
        >,
    >,
    fmp4_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl,
        >,
    >,
    frame_capture_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl,
        >,
    >,
    standard_hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_only_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fmp4_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_hls_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl {
            audio_only_hls_settings: core::default::Default::default(),
            fmp4_hls_settings: core::default::Default::default(),
            frame_capture_hls_settings: core::default::Default::default(),
            standard_hls_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_only_hls_settings` after provisioning.\n"]
    pub fn audio_only_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElAudioOnlyHlsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_only_hls_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `fmp4_hls_settings` after provisioning.\n"]
    pub fn fmp4_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFmp4HlsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fmp4_hls_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_hls_settings` after provisioning.\n"]
    pub fn frame_capture_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElFrameCaptureHlsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_hls_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_hls_settings` after provisioning.\n"]
    pub fn standard_hls_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElStandardHlsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_hls_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElDynamic {
    hls_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
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
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl {
            h265_packaging_type: core::default::Default::default(),
            name_modifier: core::default::Default::default(),
            segment_modifier: core::default::Default::default(),
            hls_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef {
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
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElHlsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl {}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl {}
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h265_packaging_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_modifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl {
            h265_packaging_type: core::default::Default::default(),
            name_modifier: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef {
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

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl {
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certficate_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
    #[doc = "Set the field `certficate_mode`.\n"]
    pub fn set_certficate_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certficate_mode = std::option::Option::Some(v.into());
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl {
            certficate_mode: core::default::Default::default(),
            connection_retry_interval: core::default::Default::default(),
            num_retries: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certficate_mode` after provisioning.\n"]
    pub fn certficate_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certficate_mode", self.base))
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
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    network_id: terrars::PrimField<f64>,
    network_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    #[doc = ""]
    pub network_id: terrars::PrimField<f64>,
    #[doc = ""]
    pub network_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl {
            network_id: self.network_id,
            network_name: self.network_name,
            rep_interval: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_sdt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_provider_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl {
            output_sdt: core::default::Default::default(),
            rep_interval: core::default::Default::default(),
            service_name: core::default::Default::default(),
            service_provider_name: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rep_interval: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    #[doc = "Set the field `rep_interval`.\n"]
    pub fn set_rep_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rep_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl {
            rep_interval: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rep_interval` after provisioning.\n"]
    pub fn rep_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rep_interval", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDynamic {
    dvb_nit_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
        >,
    >,
    dvb_sdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
        >,
    >,
    dvb_tdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {
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
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_sdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_tdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbNitSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_nit_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_sdt_settings` after provisioning.\n"]
    pub fn dvb_sdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbSdtSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_sdt_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_tdt_settings` after provisioning.\n"]
    pub fn dvb_tdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElDvbTdtSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_tdt_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElDynamic {
    m2ts_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    m2ts_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl {
            m2ts_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `m2ts_settings` after provisioning.\n"]
    pub fn m2ts_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElM2tsSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.m2ts_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl {
    destination_ref_id: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl { }

impl terrars
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl {
    #[doc = ""]
    pub destination_ref_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl {
            destination_ref_id: self.destination_ref_id,
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_ref_id` after provisioning.\n"]
    pub fn destination_ref_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ref_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_fec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_length: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl {
            column_depth: core::default::Default::default(),
            include_fec: core::default::Default::default(),
            row_length: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDynamic {
    container_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl,
        >,
    >,
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl,
        >,
    >,
    fec_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_msec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fec_output_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl {
            buffer_msec: core::default::Default::default(),
            container_settings: core::default::Default::default(),
            destination: core::default::Default::default(),
            fec_output_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef {
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
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElContainerSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElDestinationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `fec_output_settings` after provisioning.\n"]
    pub fn fec_output_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElFecOutputSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fec_output_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElDynamic {
    archive_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl,
        >,
    >,
    frame_capture_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl,
        >,
    >,
    hls_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl>,
    >,
    media_package_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl,
        >,
    >,
    ms_smooth_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl,
        >,
    >,
    multiplex_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl,
        >,
    >,
    rtmp_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl>,
    >,
    udp_output_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_output_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_package_output_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ms_smooth_output_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_output_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rtmp_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_output_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl>,
    >,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsEl,
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
                            MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_output_settings` after provisioning.\n"]
    pub fn archive_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElArchiveOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_capture_output_settings` after provisioning.\n"]
    pub fn frame_capture_output_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElFrameCaptureOutputSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `hls_output_settings` after provisioning.\n"]
    pub fn hls_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElHlsOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `media_package_output_settings` after provisioning.\n"]
    pub fn media_package_output_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMediaPackageOutputSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_package_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ms_smooth_output_settings` after provisioning.\n"]
    pub fn ms_smooth_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMsSmoothOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ms_smooth_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `multiplex_output_settings` after provisioning.\n"]
    pub fn multiplex_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElMultiplexOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rtmp_output_settings` after provisioning.\n"]
    pub fn rtmp_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRtmpOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rtmp_output_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_output_settings` after provisioning.\n"]
    pub fn udp_output_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElUdpOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.udp_output_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElDynamic {
    output_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
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
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
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
                        terrars
                        ::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsEl>,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl {
            audio_description_names: core::default::Default::default(),
            caption_description_names: core::default::Default::default(),
            output_name: core::default::Default::default(),
            video_description_name: core::default::Default::default(),
            output_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef {
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
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElOutputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElOutputGroupsElDynamic {
    output_group_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>>,
    outputs: std::option::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElOutputGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_group_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outputs: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElOutputGroupsElDynamic,
}

impl MedialiveChannelEncoderSettingsElOutputGroupsEl {
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
                        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsEl>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsEl>>,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElOutputGroupsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElOutputGroupsEl {}

impl BuildMedialiveChannelEncoderSettingsElOutputGroupsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElOutputGroupsEl {
        MedialiveChannelEncoderSettingsElOutputGroupsEl {
            name: core::default::Default::default(),
            output_group_settings: core::default::Default::default(),
            outputs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElOutputGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElOutputGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElOutputGroupsElRef {
        MedialiveChannelEncoderSettingsElOutputGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElOutputGroupsElRef {
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
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputGroupSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_group_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElOutputsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElTimecodeConfigEl {
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelEncoderSettingsElTimecodeConfigEl {
    #[doc = "Set the field `sync_threshold`.\n"]
    pub fn set_sync_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sync_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElTimecodeConfigEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElTimecodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElTimecodeConfigEl {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElTimecodeConfigEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElTimecodeConfigEl {
        MedialiveChannelEncoderSettingsElTimecodeConfigEl {
            source: self.source,
            sync_threshold: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
        MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElTimecodeConfigElRef {
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
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_interval_units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl {
            capture_interval: core::default::Default::default(),
            capture_interval_units: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef {
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
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_filter_sharpening: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strength: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {
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
::ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl {
            post_filter_sharpening: core::default::Default::default(),
            strength: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef {
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
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElDynamic {
    temporal_filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporal_filter_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {
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
                            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsEl,
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
::ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl {
            temporal_filter_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `temporal_filter_settings` after provisioning.\n"]
    pub fn temporal_filter_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElTemporalFilterSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.temporal_filter_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElDynamic {
    filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
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
        std
        ::vec
        ::Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl>,
    >,
    dynamic: MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
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
                            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl {
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

pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef {
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
    ::ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElFilterSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElDynamic {
    frame_capture_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl>,
    >,
    h264_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_capture_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h264_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
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
                            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsEl,
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
                            MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsEl,
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
}

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl {
            frame_capture_settings: core::default::Default::default(),
            h264_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frame_capture_settings` after provisioning.\n"]
    pub fn frame_capture_settings(
        &self,
    ) -> terrars
    ::ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElFrameCaptureSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_capture_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `h264_settings` after provisioning.\n"]
    pub fn h264_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElH264SettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.h264_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElVideoDescriptionsElDynamic {
    codec_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
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
    codec_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElVideoDescriptionsElDynamic,
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
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
                        terrars::BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsEl>,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelEncoderSettingsElVideoDescriptionsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
        MedialiveChannelEncoderSettingsElVideoDescriptionsEl {
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

pub struct MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
        MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElVideoDescriptionsElRef {
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
    ) -> terrars::ListRef<MedialiveChannelEncoderSettingsElVideoDescriptionsElCodecSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.codec_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelEncoderSettingsElDynamic {
    audio_descriptions: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>>,
    avail_blanking: std::option::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElAvailBlankingEl>>,
    output_groups: std::option::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElOutputGroupsEl>>,
    timecode_config: std::option::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElTimecodeConfigEl>>,
    video_descriptions: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelEncoderSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_descriptions: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avail_blanking: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElAvailBlankingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_groups: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElOutputGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timecode_config: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElTimecodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_descriptions: std::option::Option<std::vec::Vec<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>>,
    dynamic: MedialiveChannelEncoderSettingsElDynamic,
}

impl MedialiveChannelEncoderSettingsEl {
    #[doc = "Set the field `audio_descriptions`.\n"]
    pub fn set_audio_descriptions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAudioDescriptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsElAvailBlankingEl>>,
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

    #[doc = "Set the field `output_groups`.\n"]
    pub fn set_output_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsElOutputGroupsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsElTimecodeConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelEncoderSettingsElVideoDescriptionsEl>>,
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

impl terrars::ToListMappable for MedialiveChannelEncoderSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelEncoderSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelEncoderSettingsEl {}

impl BuildMedialiveChannelEncoderSettingsEl {
    pub fn build(self) -> MedialiveChannelEncoderSettingsEl {
        MedialiveChannelEncoderSettingsEl {
            audio_descriptions: core::default::Default::default(),
            avail_blanking: core::default::Default::default(),
            output_groups: core::default::Default::default(),
            timecode_config: core::default::Default::default(),
            video_descriptions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelEncoderSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelEncoderSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelEncoderSettingsElRef {
        MedialiveChannelEncoderSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelEncoderSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `avail_blanking` after provisioning.\n"]
    pub fn avail_blanking(&self) -> terrars::ListRef<MedialiveChannelEncoderSettingsElAvailBlankingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avail_blanking", self.base))
    }

    #[doc = "Get a reference to the value of field `output_groups` after provisioning.\n"]
    pub fn output_groups(&self) -> terrars::ListRef<MedialiveChannelEncoderSettingsElOutputGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `timecode_config` after provisioning.\n"]
    pub fn timecode_config(&self) -> terrars::ListRef<MedialiveChannelEncoderSettingsElTimecodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timecode_config", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
    audio_selector_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_silence_threshold_msec: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
    #[doc = "Set the field `audio_silence_threshold_msec`.\n"]
    pub fn set_audio_silence_threshold_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audio_silence_threshold_msec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
    #[doc = ""]
    pub audio_selector_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl {
            audio_selector_name: self.audio_selector_name,
            audio_silence_threshold_msec: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef {
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
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_threshold_msec: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {
    #[doc = "Set the field `input_loss_threshold_msec`.\n"]
    pub fn set_input_loss_threshold_msec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.input_loss_threshold_msec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl {
            input_loss_threshold_msec: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_loss_threshold_msec` after provisioning.\n"]
    pub fn input_loss_threshold_msec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_loss_threshold_msec", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    black_detect_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_black_threshold_msec: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {
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
::ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl {
            black_detect_threshold: core::default::Default::default(),
            video_black_threshold_msec: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef {
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
struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElDynamic {
    audio_silence_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl,
        >,
    >,
    input_loss_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl,
        >,
    >,
    video_black_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_silence_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_loss_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_black_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {
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
                            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsEl,
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
                            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsEl,
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
                            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsEl,
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
::ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl {
            audio_silence_settings: core::default::Default::default(),
            input_loss_settings: core::default::Default::default(),
            video_black_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_silence_settings` after provisioning.\n"]
    pub fn audio_silence_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElAudioSilenceSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_silence_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `input_loss_settings` after provisioning.\n"]
    pub fn input_loss_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElInputLossSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_loss_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `video_black_settings` after provisioning.\n"]
    pub fn video_black_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElVideoBlackSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_black_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElDynamic {
    failover_condition_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_condition_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElDynamic,
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
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
                            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {}

impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl {
            failover_condition_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover_condition_settings` after provisioning.\n"]
    pub fn failover_condition_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionElFailoverConditionSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_condition_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElDynamic {
    failover_condition: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_clear_time_msec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    secondary_input_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_condition: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl>>,
    dynamic: MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
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
                            MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElFailoverConditionEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    #[doc = ""]
    pub secondary_input_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl {
            error_clear_time_msec: core::default::Default::default(),
            input_preference: core::default::Default::default(),
            secondary_input_id: self.secondary_input_id,
            failover_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
        MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef {
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
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl {
    group_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl { }

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl {
    #[doc = ""]
    pub group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl {
            group_id: self.group_id,
            name: self.name,
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef {
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
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_selection_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
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
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl {
            language_code: self.language_code,
            language_selection_policy: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef {
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
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl {
    pid: terrars::PrimField<f64>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl { }

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl {
    #[doc = ""]
    pub pid: terrars::PrimField<f64>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl {
            pid: self.pid,
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pid` after provisioning.\n"]
    pub fn pid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl {
    track: terrars::PrimField<f64>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl { }

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl {
    #[doc = ""]
    pub track: terrars::PrimField<f64>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl {
            track: self.track,
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `track` after provisioning.\n"]
    pub fn track(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDynamic {
    track: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    track: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl,
        >,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {
    #[doc = "Set the field `track`.\n"]
    pub fn set_track(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElTrackEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.track = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.track = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl {
            track: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElDynamic {
    audio_hls_rendition_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl,
        >,
    >,
    audio_language_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl,
        >,
    >,
    audio_pid_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl,
        >,
    >,
    audio_track_selection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_hls_rendition_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_language_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_pid_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_track_selection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl>,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
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
                            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
    type O =
        terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl {
            audio_hls_rendition_selection: core::default::Default::default(),
            audio_language_selection: core::default::Default::default(),
            audio_pid_selection: core::default::Default::default(),
            audio_track_selection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_hls_rendition_selection` after provisioning.\n"]
    pub fn audio_hls_rendition_selection(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioHlsRenditionSelectionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_hls_rendition_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_language_selection` after provisioning.\n"]
    pub fn audio_language_selection(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioLanguageSelectionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_language_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_pid_selection` after provisioning.\n"]
    pub fn audio_pid_selection(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioPidSelectionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_pid_selection", self.base))
    }

    #[doc = "Get a reference to the value of field `audio_track_selection` after provisioning.\n"]
    pub fn audio_track_selection(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElAudioTrackSelectionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_track_selection", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElDynamic {
    selector_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selector_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl>>,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
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
                            MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl {
            name: self.name,
            selector_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef {
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
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElSelectorSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selector_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ancillary_channel_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {
    #[doc = "Set the field `source_ancillary_channel_number`.\n"]
    pub fn set_source_ancillary_channel_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.source_ancillary_channel_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl {
            source_ancillary_channel_number: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_ancillary_channel_number` after provisioning.\n"]
    pub fn source_ancillary_channel_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ancillary_channel_number", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {
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
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl {
            ocr_language: core::default::Default::default(),
            pid: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsElRef {
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
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_608_to_708: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte20_detection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_608_channel_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_608_track_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {
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

    #[doc = "Set the field `source_608_track_number`.\n"]
    pub fn set_source_608_track_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.source_608_track_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl {
            convert_608_to_708: core::default::Default::default(),
            scte20_detection: core::default::Default::default(),
            source_608_channel_number: core::default::Default::default(),
            source_608_track_number: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef {
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

    #[doc = "Get a reference to the value of field `source_608_track_number` after provisioning.\n"]
    pub fn source_608_track_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_608_track_number", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_608_to_708: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_608_channel_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {
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
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl {
            convert_608_to_708: core::default::Default::default(),
            source_608_channel_number: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef {
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
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {
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
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl {
            ocr_language: core::default::Default::default(),
            pid: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef {
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
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {
    height: terrars::PrimField<f64>,
    left_offset: terrars::PrimField<f64>,
    top_offset: terrars::PrimField<f64>,
    width: terrars::PrimField<f64>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {

}

impl terrars
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {
    #[doc = ""]
    pub height: terrars::PrimField<f64>,
    #[doc = ""]
    pub left_offset: terrars::PrimField<f64>,
    #[doc = ""]
    pub top_offset: terrars::PrimField<f64>,
    #[doc = ""]
    pub width: terrars::PrimField<f64>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl {
            height: self.height,
            left_offset: self.left_offset,
            top_offset: self.top_offset,
            width: self.width,
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef {
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
struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElDynamic {
    output_rectangle: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_rectangle: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl,
        >,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleEl,
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
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {
    type O =
        terrars
        ::BlockAssignable<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {
    pub fn build(
        self,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl {
            page_number: core::default::Default::default(),
            output_rectangle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef {
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
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElOutputRectangleElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_rectangle", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDynamic {
    ancillary_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl,
        >,
    >,
    dvb_tdt_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl,
        >,
    >,
    embedded_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl,
        >,
    >,
    scte20_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl,
        >,
    >,
    scte27_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl,
        >,
    >,
    teletext_source_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ancillary_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dvb_tdt_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedded_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte20_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scte27_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teletext_source_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl,
        >,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsEl,
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl {
            ancillary_source_settings: core::default::Default::default(),
            dvb_tdt_settings: core::default::Default::default(),
            embedded_source_settings: core::default::Default::default(),
            scte20_source_settings: core::default::Default::default(),
            scte27_source_settings: core::default::Default::default(),
            teletext_source_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ancillary_source_settings` after provisioning.\n"]
    pub fn ancillary_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElAncillarySourceSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ancillary_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dvb_tdt_settings` after provisioning.\n"]
    pub fn dvb_tdt_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElDvbTdtSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dvb_tdt_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `embedded_source_settings` after provisioning.\n"]
    pub fn embedded_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElEmbeddedSourceSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.embedded_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `scte20_source_settings` after provisioning.\n"]
    pub fn scte20_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte20SourceSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scte20_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `scte27_source_settings` after provisioning.\n"]
    pub fn scte27_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElScte27SourceSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scte27_source_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `teletext_source_settings` after provisioning.\n"]
    pub fn teletext_source_settings(
        &self,
    ) -> terrars
    ::ListRef<
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElTeletextSourceSettingsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teletext_source_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElDynamic {
    selector_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selector_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl>>,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
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
                            MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl {
            language_code: core::default::Default::default(),
            name: self.name,
            selector_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef {
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
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElSelectorSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selector_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
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

impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
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
::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl {
            bandwidth: core::default::Default::default(),
            buffer_segments: core::default::Default::default(),
            retries: core::default::Default::default(),
            retry_interval: core::default::Default::default(),
            scte35_source: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef {
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
struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElDynamic {
    hls_input_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_validation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hls_input_settings: std
    ::option
    ::Option<
        std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl>,
    >,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
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
                            MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsEl,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl {
            server_validation: core::default::Default::default(),
            hls_input_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef {
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
    ) -> terrars
    ::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElHlsInputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hls_input_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_space: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color_space_usage: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
        MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl {
            color_space: core::default::Default::default(),
            color_space_usage: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef {
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
struct MedialiveChannelInputAttachmentsElInputSettingsElDynamic {
    audio_selector: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>>,
    caption_selector: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>>,
    network_input_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>>,
    video_selector: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsElInputSettingsEl {
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
    audio_selector: std::option::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caption_selector: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_input_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_selector: std::option::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>>,
    dynamic: MedialiveChannelInputAttachmentsElInputSettingsElDynamic,
}

impl MedialiveChannelInputAttachmentsElInputSettingsEl {
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
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorEl>,
                    >,
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
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorEl>,
                    >,
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
                        terrars
                        ::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsEl>,
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
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorEl>,
                    >,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsElInputSettingsEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsElInputSettingsEl {}

impl BuildMedialiveChannelInputAttachmentsElInputSettingsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsElInputSettingsEl {
        MedialiveChannelInputAttachmentsElInputSettingsEl {
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

pub struct MedialiveChannelInputAttachmentsElInputSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElInputSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveChannelInputAttachmentsElInputSettingsElRef {
        MedialiveChannelInputAttachmentsElInputSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElInputSettingsElRef {
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
    pub fn audio_selector(
        &self,
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElAudioSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `caption_selector` after provisioning.\n"]
    pub fn caption_selector(
        &self,
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElCaptionSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.caption_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `network_input_settings` after provisioning.\n"]
    pub fn network_input_settings(
        &self,
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElNetworkInputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_input_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `video_selector` after provisioning.\n"]
    pub fn video_selector(
        &self,
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElVideoSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_selector", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelInputAttachmentsElDynamic {
    automatic_input_failover_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>>,
    input_settings: std::option::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsElInputSettingsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveChannelInputAttachmentsEl {
    input_attachment_name: terrars::PrimField<std::string::String>,
    input_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_input_failover_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_settings: std::option::Option<std::vec::Vec<MedialiveChannelInputAttachmentsElInputSettingsEl>>,
    dynamic: MedialiveChannelInputAttachmentsElDynamic,
}

impl MedialiveChannelInputAttachmentsEl {
    #[doc = "Set the field `automatic_input_failover_settings`.\n"]
    pub fn set_automatic_input_failover_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsEl>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveChannelInputAttachmentsElInputSettingsEl>>,
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

impl terrars::ToListMappable for MedialiveChannelInputAttachmentsEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputAttachmentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputAttachmentsEl {
    #[doc = ""]
    pub input_attachment_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub input_id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputAttachmentsEl {
    pub fn build(self) -> MedialiveChannelInputAttachmentsEl {
        MedialiveChannelInputAttachmentsEl {
            input_attachment_name: self.input_attachment_name,
            input_id: self.input_id,
            automatic_input_failover_settings: core::default::Default::default(),
            input_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelInputAttachmentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputAttachmentsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelInputAttachmentsElRef {
        MedialiveChannelInputAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputAttachmentsElRef {
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
    ) -> terrars::ListRef<MedialiveChannelInputAttachmentsElAutomaticInputFailoverSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_input_failover_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `input_settings` after provisioning.\n"]
    pub fn input_settings(&self) -> terrars::ListRef<MedialiveChannelInputAttachmentsElInputSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveChannelInputSpecificationEl {
    codec: terrars::PrimField<std::string::String>,
    input_resolution: terrars::PrimField<std::string::String>,
    maximum_bitrate: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelInputSpecificationEl { }

impl terrars::ToListMappable for MedialiveChannelInputSpecificationEl {
    type O = terrars::BlockAssignable<MedialiveChannelInputSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelInputSpecificationEl {
    #[doc = ""]
    pub codec: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub input_resolution: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub maximum_bitrate: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelInputSpecificationEl {
    pub fn build(self) -> MedialiveChannelInputSpecificationEl {
        MedialiveChannelInputSpecificationEl {
            codec: self.codec,
            input_resolution: self.input_resolution,
            maximum_bitrate: self.maximum_bitrate,
        }
    }
}

pub struct MedialiveChannelInputSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelInputSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelInputSpecificationElRef {
        MedialiveChannelInputSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelInputSpecificationElRef {
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
pub struct MedialiveChannelMaintenanceEl {
    maintenance_day: terrars::PrimField<std::string::String>,
    maintenance_start_time: terrars::PrimField<std::string::String>,
}

impl MedialiveChannelMaintenanceEl { }

impl terrars::ToListMappable for MedialiveChannelMaintenanceEl {
    type O = terrars::BlockAssignable<MedialiveChannelMaintenanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelMaintenanceEl {
    #[doc = ""]
    pub maintenance_day: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub maintenance_start_time: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveChannelMaintenanceEl {
    pub fn build(self) -> MedialiveChannelMaintenanceEl {
        MedialiveChannelMaintenanceEl {
            maintenance_day: self.maintenance_day,
            maintenance_start_time: self.maintenance_start_time,
        }
    }
}

pub struct MedialiveChannelMaintenanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelMaintenanceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelMaintenanceElRef {
        MedialiveChannelMaintenanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelMaintenanceElRef {
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
pub struct MedialiveChannelTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelTimeoutsEl {
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

impl terrars::ToListMappable for MedialiveChannelTimeoutsEl {
    type O = terrars::BlockAssignable<MedialiveChannelTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelTimeoutsEl {}

impl BuildMedialiveChannelTimeoutsEl {
    pub fn build(self) -> MedialiveChannelTimeoutsEl {
        MedialiveChannelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MedialiveChannelTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelTimeoutsElRef {
        MedialiveChannelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelTimeoutsElRef {
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
pub struct MedialiveChannelVpcEl {
    public_address_allocation_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl MedialiveChannelVpcEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveChannelVpcEl {
    type O = terrars::BlockAssignable<MedialiveChannelVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveChannelVpcEl {
    #[doc = ""]
    pub public_address_allocation_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMedialiveChannelVpcEl {
    pub fn build(self) -> MedialiveChannelVpcEl {
        MedialiveChannelVpcEl {
            public_address_allocation_ids: self.public_address_allocation_ids,
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct MedialiveChannelVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveChannelVpcElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveChannelVpcElRef {
        MedialiveChannelVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveChannelVpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `public_address_allocation_ids` after provisioning.\n"]
    pub fn public_address_allocation_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_address_allocation_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveChannelDynamic {
    cdi_input_specification: std::option::Option<terrars::DynamicBlock<MedialiveChannelCdiInputSpecificationEl>>,
    destinations: std::option::Option<terrars::DynamicBlock<MedialiveChannelDestinationsEl>>,
    encoder_settings: std::option::Option<terrars::DynamicBlock<MedialiveChannelEncoderSettingsEl>>,
    input_attachments: std::option::Option<terrars::DynamicBlock<MedialiveChannelInputAttachmentsEl>>,
    input_specification: std::option::Option<terrars::DynamicBlock<MedialiveChannelInputSpecificationEl>>,
    maintenance: std::option::Option<terrars::DynamicBlock<MedialiveChannelMaintenanceEl>>,
    vpc: std::option::Option<terrars::DynamicBlock<MedialiveChannelVpcEl>>,
}
