use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IvsRecordingConfigurationData {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_reconnect_window_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configuration: std::option::Option<std::vec::Vec<IvsRecordingConfigurationDestinationConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbnail_configuration: std::option::Option<std::vec::Vec<IvsRecordingConfigurationThumbnailConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IvsRecordingConfigurationTimeoutsEl>,
    dynamic: IvsRecordingConfigurationDynamic,
}

struct IvsRecordingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IvsRecordingConfigurationData>,
}

#[derive(Clone)]
pub struct IvsRecordingConfiguration(std::rc::Rc<IvsRecordingConfiguration_>);

impl IvsRecordingConfiguration {
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

    #[doc = "Set the field `recording_reconnect_window_seconds`.\n"]
    pub fn set_recording_reconnect_window_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().recording_reconnect_window_seconds = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IvsRecordingConfigurationDestinationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thumbnail_configuration`.\n"]
    pub fn set_thumbnail_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IvsRecordingConfigurationThumbnailConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnail_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnail_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IvsRecordingConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_reconnect_window_seconds` after provisioning.\n"]
    pub fn recording_reconnect_window_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.recording_reconnect_window_seconds", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(&self) -> terrars::ListRef<IvsRecordingConfigurationDestinationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnail_configuration` after provisioning.\n"]
    pub fn thumbnail_configuration(&self) -> terrars::ListRef<IvsRecordingConfigurationThumbnailConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thumbnail_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvsRecordingConfigurationTimeoutsElRef {
        IvsRecordingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IvsRecordingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IvsRecordingConfiguration { }

impl terrars::ToListMappable for IvsRecordingConfiguration {
    type O = terrars::ListRef<IvsRecordingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IvsRecordingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ivs_recording_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIvsRecordingConfiguration {
    pub tf_id: std::string::String,
}

impl BuildIvsRecordingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> IvsRecordingConfiguration {
        let out = IvsRecordingConfiguration(std::rc::Rc::new(IvsRecordingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IvsRecordingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                recording_reconnect_window_seconds: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_configuration: core::default::Default::default(),
                thumbnail_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IvsRecordingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvsRecordingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IvsRecordingConfigurationRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_reconnect_window_seconds` after provisioning.\n"]
    pub fn recording_reconnect_window_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.recording_reconnect_window_seconds", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(&self) -> terrars::ListRef<IvsRecordingConfigurationDestinationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnail_configuration` after provisioning.\n"]
    pub fn thumbnail_configuration(&self) -> terrars::ListRef<IvsRecordingConfigurationThumbnailConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thumbnail_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvsRecordingConfigurationTimeoutsElRef {
        IvsRecordingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationDestinationConfigurationElS3El {
    bucket_name: terrars::PrimField<std::string::String>,
}

impl IvsRecordingConfigurationDestinationConfigurationElS3El { }

impl terrars::ToListMappable for IvsRecordingConfigurationDestinationConfigurationElS3El {
    type O = terrars::BlockAssignable<IvsRecordingConfigurationDestinationConfigurationElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationDestinationConfigurationElS3El {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildIvsRecordingConfigurationDestinationConfigurationElS3El {
    pub fn build(self) -> IvsRecordingConfigurationDestinationConfigurationElS3El {
        IvsRecordingConfigurationDestinationConfigurationElS3El { bucket_name: self.bucket_name }
    }
}

pub struct IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
        IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationDestinationConfigurationElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IvsRecordingConfigurationDestinationConfigurationElDynamic {
    s3: std::option::Option<terrars::DynamicBlock<IvsRecordingConfigurationDestinationConfigurationElS3El>>,
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationDestinationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<IvsRecordingConfigurationDestinationConfigurationElS3El>>,
    dynamic: IvsRecordingConfigurationDestinationConfigurationElDynamic,
}

impl IvsRecordingConfigurationDestinationConfigurationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IvsRecordingConfigurationDestinationConfigurationElS3El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IvsRecordingConfigurationDestinationConfigurationEl {
    type O = terrars::BlockAssignable<IvsRecordingConfigurationDestinationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationDestinationConfigurationEl {}

impl BuildIvsRecordingConfigurationDestinationConfigurationEl {
    pub fn build(self) -> IvsRecordingConfigurationDestinationConfigurationEl {
        IvsRecordingConfigurationDestinationConfigurationEl {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IvsRecordingConfigurationDestinationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvsRecordingConfigurationDestinationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvsRecordingConfigurationDestinationConfigurationElRef {
        IvsRecordingConfigurationDestinationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationDestinationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<IvsRecordingConfigurationDestinationConfigurationElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationThumbnailConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl IvsRecordingConfigurationThumbnailConfigurationEl {
    #[doc = "Set the field `recording_mode`.\n"]
    pub fn set_recording_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recording_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_interval_seconds`.\n"]
    pub fn set_target_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_interval_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IvsRecordingConfigurationThumbnailConfigurationEl {
    type O = terrars::BlockAssignable<IvsRecordingConfigurationThumbnailConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationThumbnailConfigurationEl {}

impl BuildIvsRecordingConfigurationThumbnailConfigurationEl {
    pub fn build(self) -> IvsRecordingConfigurationThumbnailConfigurationEl {
        IvsRecordingConfigurationThumbnailConfigurationEl {
            recording_mode: core::default::Default::default(),
            target_interval_seconds: core::default::Default::default(),
        }
    }
}

pub struct IvsRecordingConfigurationThumbnailConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvsRecordingConfigurationThumbnailConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IvsRecordingConfigurationThumbnailConfigurationElRef {
        IvsRecordingConfigurationThumbnailConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationThumbnailConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recording_mode` after provisioning.\n"]
    pub fn recording_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recording_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `target_interval_seconds` after provisioning.\n"]
    pub fn target_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_interval_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct IvsRecordingConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IvsRecordingConfigurationTimeoutsEl {
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
}

impl terrars::ToListMappable for IvsRecordingConfigurationTimeoutsEl {
    type O = terrars::BlockAssignable<IvsRecordingConfigurationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvsRecordingConfigurationTimeoutsEl {}

impl BuildIvsRecordingConfigurationTimeoutsEl {
    pub fn build(self) -> IvsRecordingConfigurationTimeoutsEl {
        IvsRecordingConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct IvsRecordingConfigurationTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IvsRecordingConfigurationTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IvsRecordingConfigurationTimeoutsElRef {
        IvsRecordingConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvsRecordingConfigurationTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct IvsRecordingConfigurationDynamic {
    destination_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IvsRecordingConfigurationDestinationConfigurationEl>>,
    thumbnail_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IvsRecordingConfigurationThumbnailConfigurationEl>>,
}
