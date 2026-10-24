use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MedialiveInputData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_security_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<MedialiveInputDestinationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_devices: std::option::Option<std::vec::Vec<MedialiveInputInputDevicesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_connect_flows: std::option::Option<std::vec::Vec<MedialiveInputMediaConnectFlowsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<std::vec::Vec<MedialiveInputSourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MedialiveInputTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<MedialiveInputVpcEl>>,
    dynamic: MedialiveInputDynamic,
}

struct MedialiveInput_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MedialiveInputData>,
}

#[derive(Clone)]
pub struct MedialiveInput(std::rc::Rc<MedialiveInput_>);

impl MedialiveInput {
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

    #[doc = "Set the field `input_security_groups`.\n"]
    pub fn set_input_security_groups(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().input_security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveInputDestinationsEl>>,
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

    #[doc = "Set the field `input_devices`.\n"]
    pub fn set_input_devices(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveInputInputDevicesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_devices = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_devices = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `media_connect_flows`.\n"]
    pub fn set_media_connect_flows(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveInputMediaConnectFlowsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_connect_flows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.media_connect_flows = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(self, v: impl std::convert::Into<terrars::BlockAssignable<MedialiveInputSourcesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MedialiveInputTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl std::convert::Into<terrars::BlockAssignable<MedialiveInputVpcEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_channels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_partner_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_security_groups` after provisioning.\n"]
    pub fn input_security_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveInputTimeoutsElRef {
        MedialiveInputTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MedialiveInputVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

impl terrars::Referable for MedialiveInput {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MedialiveInput { }

impl terrars::ToListMappable for MedialiveInput {
    type O = terrars::ListRef<MedialiveInputRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MedialiveInput_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_medialive_input".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMedialiveInput {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveInput {
    pub fn build(self, stack: &mut terrars::Stack) -> MedialiveInput {
        let out = MedialiveInput(std::rc::Rc::new(MedialiveInput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MedialiveInputData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                input_security_groups: core::default::Default::default(),
                name: self.name,
                role_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                destinations: core::default::Default::default(),
                input_devices: core::default::Default::default(),
                media_connect_flows: core::default::Default::default(),
                sources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MedialiveInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MedialiveInputRef {
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

    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_channels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_partner_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_security_groups` after provisioning.\n"]
    pub fn input_security_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveInputTimeoutsElRef {
        MedialiveInputTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<MedialiveInputVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MedialiveInputDestinationsEl {
    stream_name: terrars::PrimField<std::string::String>,
}

impl MedialiveInputDestinationsEl { }

impl terrars::ToListMappable for MedialiveInputDestinationsEl {
    type O = terrars::BlockAssignable<MedialiveInputDestinationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveInputDestinationsEl {
    #[doc = ""]
    pub stream_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveInputDestinationsEl {
    pub fn build(self) -> MedialiveInputDestinationsEl {
        MedialiveInputDestinationsEl { stream_name: self.stream_name }
    }
}

pub struct MedialiveInputDestinationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputDestinationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveInputDestinationsElRef {
        MedialiveInputDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveInputDestinationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveInputInputDevicesEl {
    id: terrars::PrimField<std::string::String>,
}

impl MedialiveInputInputDevicesEl { }

impl terrars::ToListMappable for MedialiveInputInputDevicesEl {
    type O = terrars::BlockAssignable<MedialiveInputInputDevicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveInputInputDevicesEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveInputInputDevicesEl {
    pub fn build(self) -> MedialiveInputInputDevicesEl {
        MedialiveInputInputDevicesEl { id: self.id }
    }
}

pub struct MedialiveInputInputDevicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputInputDevicesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveInputInputDevicesElRef {
        MedialiveInputInputDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveInputInputDevicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveInputMediaConnectFlowsEl {
    flow_arn: terrars::PrimField<std::string::String>,
}

impl MedialiveInputMediaConnectFlowsEl { }

impl terrars::ToListMappable for MedialiveInputMediaConnectFlowsEl {
    type O = terrars::BlockAssignable<MedialiveInputMediaConnectFlowsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveInputMediaConnectFlowsEl {
    #[doc = ""]
    pub flow_arn: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveInputMediaConnectFlowsEl {
    pub fn build(self) -> MedialiveInputMediaConnectFlowsEl {
        MedialiveInputMediaConnectFlowsEl { flow_arn: self.flow_arn }
    }
}

pub struct MedialiveInputMediaConnectFlowsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputMediaConnectFlowsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveInputMediaConnectFlowsElRef {
        MedialiveInputMediaConnectFlowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveInputMediaConnectFlowsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_arn` after provisioning.\n"]
    pub fn flow_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveInputSourcesEl {
    password_param: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl MedialiveInputSourcesEl { }

impl terrars::ToListMappable for MedialiveInputSourcesEl {
    type O = terrars::BlockAssignable<MedialiveInputSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveInputSourcesEl {
    #[doc = ""]
    pub password_param: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveInputSourcesEl {
    pub fn build(self) -> MedialiveInputSourcesEl {
        MedialiveInputSourcesEl {
            password_param: self.password_param,
            url: self.url,
            username: self.username,
        }
    }
}

pub struct MedialiveInputSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputSourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveInputSourcesElRef {
        MedialiveInputSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveInputSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
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

#[derive(Serialize)]
pub struct MedialiveInputTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveInputTimeoutsEl {
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

impl terrars::ToListMappable for MedialiveInputTimeoutsEl {
    type O = terrars::BlockAssignable<MedialiveInputTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveInputTimeoutsEl {}

impl BuildMedialiveInputTimeoutsEl {
    pub fn build(self) -> MedialiveInputTimeoutsEl {
        MedialiveInputTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MedialiveInputTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveInputTimeoutsElRef {
        MedialiveInputTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveInputTimeoutsElRef {
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
pub struct MedialiveInputVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl MedialiveInputVpcEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveInputVpcEl {
    type O = terrars::BlockAssignable<MedialiveInputVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveInputVpcEl {
    #[doc = ""]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMedialiveInputVpcEl {
    pub fn build(self) -> MedialiveInputVpcEl {
        MedialiveInputVpcEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct MedialiveInputVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveInputVpcElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveInputVpcElRef {
        MedialiveInputVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveInputVpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
struct MedialiveInputDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<MedialiveInputDestinationsEl>>,
    input_devices: std::option::Option<terrars::DynamicBlock<MedialiveInputInputDevicesEl>>,
    media_connect_flows: std::option::Option<terrars::DynamicBlock<MedialiveInputMediaConnectFlowsEl>>,
    sources: std::option::Option<terrars::DynamicBlock<MedialiveInputSourcesEl>>,
    vpc: std::option::Option<terrars::DynamicBlock<MedialiveInputVpcEl>>,
}
