#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElastictranscoderPresetData_ {
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
    container: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_codec_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio: std::option::Option<std::vec::Vec<ElastictranscoderPreset_Audio>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_codec_options: std::option::Option<std::vec::Vec<ElastictranscoderPreset_AudioCodecOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbnails: std::option::Option<std::vec::Vec<ElastictranscoderPreset_Thumbnails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video: std::option::Option<std::vec::Vec<ElastictranscoderPreset_Video>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_watermarks: std::option::Option<std::vec::Vec<ElastictranscoderPreset_VideoWatermarks>>,
    dynamic: ElastictranscoderPresetDynamic,
}

struct ElastictranscoderPreset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElastictranscoderPresetData_>,
}

#[derive(Clone)]
pub struct ElastictranscoderPreset(std::rc::Rc<ElastictranscoderPreset_>);

impl ElastictranscoderPreset {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `video_codec_options`.\n"]
    pub fn set_video_codec_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().video_codec_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio`.\n"]
    pub fn set_audio(self, v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPreset_Audio>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `audio_codec_options`.\n"]
    pub fn set_audio_codec_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPreset_AudioCodecOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio_codec_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio_codec_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thumbnails`.\n"]
    pub fn set_thumbnails(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPreset_Thumbnails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnails = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnails = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video`.\n"]
    pub fn set_video(self, v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPreset_Video>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().video = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.video = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_watermarks`.\n"]
    pub fn set_video_watermarks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPreset_VideoWatermarks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().video_watermarks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.video_watermarks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `video_codec_options` after provisioning.\n"]
    pub fn video_codec_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.video_codec_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\n"]
    pub fn audio(&self) -> terrars::ListRef<ElastictranscoderPreset_AudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio_codec_options` after provisioning.\n"]
    pub fn audio_codec_options(&self) -> terrars::ListRef<ElastictranscoderPreset_AudioCodecOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_codec_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnails` after provisioning.\n"]
    pub fn thumbnails(&self) -> terrars::ListRef<ElastictranscoderPreset_ThumbnailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thumbnails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `video` after provisioning.\n"]
    pub fn video(&self) -> terrars::ListRef<ElastictranscoderPreset_VideoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video", self.extract_ref()))
    }
}

impl terrars::Referable for ElastictranscoderPreset {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ElastictranscoderPreset { }

impl terrars::ToListMappable for ElastictranscoderPreset {
    type O = terrars::ListRef<ElastictranscoderPresetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ElastictranscoderPreset_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_elastictranscoder_preset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElastictranscoderPreset {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub container: terrars::PrimField<std::string::String>,
}

impl BuildElastictranscoderPreset {
    pub fn build(self, stack: &mut terrars::Stack) -> ElastictranscoderPreset {
        let out = ElastictranscoderPreset(std::rc::Rc::new(ElastictranscoderPreset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ElastictranscoderPresetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container: self.container,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                video_codec_options: core::default::Default::default(),
                audio: core::default::Default::default(),
                audio_codec_options: core::default::Default::default(),
                thumbnails: core::default::Default::default(),
                video: core::default::Default::default(),
                video_watermarks: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElastictranscoderPresetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPresetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ElastictranscoderPresetRef {
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

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `video_codec_options` after provisioning.\n"]
    pub fn video_codec_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.video_codec_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\n"]
    pub fn audio(&self) -> terrars::ListRef<ElastictranscoderPreset_AudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio_codec_options` after provisioning.\n"]
    pub fn audio_codec_options(&self) -> terrars::ListRef<ElastictranscoderPreset_AudioCodecOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_codec_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnails` after provisioning.\n"]
    pub fn thumbnails(&self) -> terrars::ListRef<ElastictranscoderPreset_ThumbnailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thumbnails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `video` after provisioning.\n"]
    pub fn video(&self) -> terrars::ListRef<ElastictranscoderPreset_VideoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ElastictranscoderPresetDynamic {
    audio: std::option::Option<terrars::DynamicBlock<ElastictranscoderPreset_Audio>>,
    audio_codec_options: std::option::Option<terrars::DynamicBlock<ElastictranscoderPreset_AudioCodecOptions>>,
    thumbnails: std::option::Option<terrars::DynamicBlock<ElastictranscoderPreset_Thumbnails>>,
    video: std::option::Option<terrars::DynamicBlock<ElastictranscoderPreset_Video>>,
    video_watermarks: std::option::Option<terrars::DynamicBlock<ElastictranscoderPreset_VideoWatermarks>>,
}

#[derive(Serialize)]
pub struct ElastictranscoderPreset_Audio {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_packing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bit_rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channels: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPreset_Audio {
    #[doc = "Set the field `audio_packing_mode`.\n"]
    pub fn set_audio_packing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_packing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bit_rate`.\n"]
    pub fn set_bit_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bit_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `channels`.\n"]
    pub fn set_channels(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `codec`.\n"]
    pub fn set_codec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.codec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate`.\n"]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPreset_Audio {
    type O = terrars::BlockAssignable<ElastictranscoderPreset_Audio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPreset_Audio {}

impl BuildElastictranscoderPreset_Audio {
    pub fn build(self) -> ElastictranscoderPreset_Audio {
        ElastictranscoderPreset_Audio {
            audio_packing_mode: core::default::Default::default(),
            bit_rate: core::default::Default::default(),
            channels: core::default::Default::default(),
            codec: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElastictranscoderPreset_AudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPreset_AudioRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPreset_AudioRef {
        ElastictranscoderPreset_AudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPreset_AudioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_packing_mode` after provisioning.\n"]
    pub fn audio_packing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_packing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `bit_rate` after provisioning.\n"]
    pub fn bit_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bit_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `channels` after provisioning.\n"]
    pub fn channels(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channels", self.base))
    }

    #[doc = "Get a reference to the value of field `codec` after provisioning.\n"]
    pub fn codec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_rate` after provisioning.\n"]
    pub fn sample_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPreset_AudioCodecOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bit_depth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bit_order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPreset_AudioCodecOptions {
    #[doc = "Set the field `bit_depth`.\n"]
    pub fn set_bit_depth(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bit_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bit_order`.\n"]
    pub fn set_bit_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bit_order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signed`.\n"]
    pub fn set_signed(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPreset_AudioCodecOptions {
    type O = terrars::BlockAssignable<ElastictranscoderPreset_AudioCodecOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPreset_AudioCodecOptions {}

impl BuildElastictranscoderPreset_AudioCodecOptions {
    pub fn build(self) -> ElastictranscoderPreset_AudioCodecOptions {
        ElastictranscoderPreset_AudioCodecOptions {
            bit_depth: core::default::Default::default(),
            bit_order: core::default::Default::default(),
            profile: core::default::Default::default(),
            signed: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElastictranscoderPreset_AudioCodecOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPreset_AudioCodecOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPreset_AudioCodecOptionsRef {
        ElastictranscoderPreset_AudioCodecOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPreset_AudioCodecOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bit_depth` after provisioning.\n"]
    pub fn bit_depth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bit_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `bit_order` after provisioning.\n"]
    pub fn bit_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bit_order", self.base))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile", self.base))
    }

    #[doc = "Get a reference to the value of field `signed` after provisioning.\n"]
    pub fn signed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPreset_Thumbnails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aspect_ratio: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_height: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_width: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    padding_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolution: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sizing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPreset_Thumbnails {
    #[doc = "Set the field `aspect_ratio`.\n"]
    pub fn set_aspect_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aspect_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_height`.\n"]
    pub fn set_max_height(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_width`.\n"]
    pub fn set_max_width(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `padding_policy`.\n"]
    pub fn set_padding_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.padding_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resolution`.\n"]
    pub fn set_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sizing_policy`.\n"]
    pub fn set_sizing_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sizing_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPreset_Thumbnails {
    type O = terrars::BlockAssignable<ElastictranscoderPreset_Thumbnails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPreset_Thumbnails {}

impl BuildElastictranscoderPreset_Thumbnails {
    pub fn build(self) -> ElastictranscoderPreset_Thumbnails {
        ElastictranscoderPreset_Thumbnails {
            aspect_ratio: core::default::Default::default(),
            format: core::default::Default::default(),
            interval: core::default::Default::default(),
            max_height: core::default::Default::default(),
            max_width: core::default::Default::default(),
            padding_policy: core::default::Default::default(),
            resolution: core::default::Default::default(),
            sizing_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElastictranscoderPreset_ThumbnailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPreset_ThumbnailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPreset_ThumbnailsRef {
        ElastictranscoderPreset_ThumbnailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPreset_ThumbnailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aspect_ratio` after provisioning.\n"]
    pub fn aspect_ratio(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aspect_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `max_height` after provisioning.\n"]
    pub fn max_height(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_height", self.base))
    }

    #[doc = "Get a reference to the value of field `max_width` after provisioning.\n"]
    pub fn max_width(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_width", self.base))
    }

    #[doc = "Get a reference to the value of field `padding_policy` after provisioning.\n"]
    pub fn padding_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.padding_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `sizing_policy` after provisioning.\n"]
    pub fn sizing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sizing_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPreset_Video {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aspect_ratio: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bit_rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_aspect_ratio: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_gop: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyframes_max_dist: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_frame_rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_height: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_width: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    padding_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolution: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sizing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPreset_Video {
    #[doc = "Set the field `aspect_ratio`.\n"]
    pub fn set_aspect_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aspect_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bit_rate`.\n"]
    pub fn set_bit_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bit_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `codec`.\n"]
    pub fn set_codec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.codec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_aspect_ratio`.\n"]
    pub fn set_display_aspect_ratio(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.display_aspect_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_gop`.\n"]
    pub fn set_fixed_gop(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_gop = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `frame_rate`.\n"]
    pub fn set_frame_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frame_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keyframes_max_dist`.\n"]
    pub fn set_keyframes_max_dist(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.keyframes_max_dist = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_frame_rate`.\n"]
    pub fn set_max_frame_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_frame_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_height`.\n"]
    pub fn set_max_height(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_width`.\n"]
    pub fn set_max_width(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `padding_policy`.\n"]
    pub fn set_padding_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.padding_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resolution`.\n"]
    pub fn set_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sizing_policy`.\n"]
    pub fn set_sizing_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sizing_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPreset_Video {
    type O = terrars::BlockAssignable<ElastictranscoderPreset_Video>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPreset_Video {}

impl BuildElastictranscoderPreset_Video {
    pub fn build(self) -> ElastictranscoderPreset_Video {
        ElastictranscoderPreset_Video {
            aspect_ratio: core::default::Default::default(),
            bit_rate: core::default::Default::default(),
            codec: core::default::Default::default(),
            display_aspect_ratio: core::default::Default::default(),
            fixed_gop: core::default::Default::default(),
            frame_rate: core::default::Default::default(),
            keyframes_max_dist: core::default::Default::default(),
            max_frame_rate: core::default::Default::default(),
            max_height: core::default::Default::default(),
            max_width: core::default::Default::default(),
            padding_policy: core::default::Default::default(),
            resolution: core::default::Default::default(),
            sizing_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElastictranscoderPreset_VideoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPreset_VideoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPreset_VideoRef {
        ElastictranscoderPreset_VideoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPreset_VideoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aspect_ratio` after provisioning.\n"]
    pub fn aspect_ratio(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aspect_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `bit_rate` after provisioning.\n"]
    pub fn bit_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bit_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `codec` after provisioning.\n"]
    pub fn codec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codec", self.base))
    }

    #[doc = "Get a reference to the value of field `display_aspect_ratio` after provisioning.\n"]
    pub fn display_aspect_ratio(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_aspect_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_gop` after provisioning.\n"]
    pub fn fixed_gop(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_gop", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_rate` after provisioning.\n"]
    pub fn frame_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frame_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `keyframes_max_dist` after provisioning.\n"]
    pub fn keyframes_max_dist(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyframes_max_dist", self.base))
    }

    #[doc = "Get a reference to the value of field `max_frame_rate` after provisioning.\n"]
    pub fn max_frame_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_frame_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `max_height` after provisioning.\n"]
    pub fn max_height(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_height", self.base))
    }

    #[doc = "Get a reference to the value of field `max_width` after provisioning.\n"]
    pub fn max_width(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_width", self.base))
    }

    #[doc = "Get a reference to the value of field `padding_policy` after provisioning.\n"]
    pub fn padding_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.padding_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `resolution` after provisioning.\n"]
    pub fn resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `sizing_policy` after provisioning.\n"]
    pub fn sizing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sizing_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPreset_VideoWatermarks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_align: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_offset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_height: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_width: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opacity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sizing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertical_align: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertical_offset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPreset_VideoWatermarks {
    #[doc = "Set the field `horizontal_align`.\n"]
    pub fn set_horizontal_align(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.horizontal_align = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `horizontal_offset`.\n"]
    pub fn set_horizontal_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.horizontal_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_height`.\n"]
    pub fn set_max_height(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_width`.\n"]
    pub fn set_max_width(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opacity`.\n"]
    pub fn set_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sizing_policy`.\n"]
    pub fn set_sizing_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sizing_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vertical_align`.\n"]
    pub fn set_vertical_align(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vertical_align = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vertical_offset`.\n"]
    pub fn set_vertical_offset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vertical_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPreset_VideoWatermarks {
    type O = terrars::BlockAssignable<ElastictranscoderPreset_VideoWatermarks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPreset_VideoWatermarks {}

impl BuildElastictranscoderPreset_VideoWatermarks {
    pub fn build(self) -> ElastictranscoderPreset_VideoWatermarks {
        ElastictranscoderPreset_VideoWatermarks {
            horizontal_align: core::default::Default::default(),
            horizontal_offset: core::default::Default::default(),
            id: core::default::Default::default(),
            max_height: core::default::Default::default(),
            max_width: core::default::Default::default(),
            opacity: core::default::Default::default(),
            sizing_policy: core::default::Default::default(),
            target: core::default::Default::default(),
            vertical_align: core::default::Default::default(),
            vertical_offset: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElastictranscoderPreset_VideoWatermarksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPreset_VideoWatermarksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPreset_VideoWatermarksRef {
        ElastictranscoderPreset_VideoWatermarksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPreset_VideoWatermarksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `horizontal_align` after provisioning.\n"]
    pub fn horizontal_align(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.horizontal_align", self.base))
    }

    #[doc = "Get a reference to the value of field `horizontal_offset` after provisioning.\n"]
    pub fn horizontal_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.horizontal_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_height` after provisioning.\n"]
    pub fn max_height(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_height", self.base))
    }

    #[doc = "Get a reference to the value of field `max_width` after provisioning.\n"]
    pub fn max_width(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_width", self.base))
    }

    #[doc = "Get a reference to the value of field `opacity` after provisioning.\n"]
    pub fn opacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `sizing_policy` after provisioning.\n"]
    pub fn sizing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sizing_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `vertical_align` after provisioning.\n"]
    pub fn vertical_align(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vertical_align", self.base))
    }

    #[doc = "Get a reference to the value of field `vertical_offset` after provisioning.\n"]
    pub fn vertical_offset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vertical_offset", self.base))
    }
}
