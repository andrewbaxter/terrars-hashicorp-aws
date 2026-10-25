#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RekognitionStreamProcessorData_ {
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
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_sharing_preference: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_DataSharingPreference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Input>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_channel: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_NotificationChannel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Output>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions_of_interest: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_RegionsOfInterest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RekognitionStreamProcessor_Timeouts>,
    dynamic: RekognitionStreamProcessorDynamic,
}

struct RekognitionStreamProcessor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RekognitionStreamProcessorData_>,
}

#[derive(Clone)]
pub struct RekognitionStreamProcessor(std::rc::Rc<RekognitionStreamProcessor_>);

impl RekognitionStreamProcessor {
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

    #[doc =
        "Set the field `kms_key_id`.\nThe identifier for your AWS Key Management Service key (AWS KMS key). You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN."]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_sharing_preference`.\n"]
    pub fn set_data_sharing_preference(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_DataSharingPreference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_sharing_preference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_sharing_preference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Input>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_channel`.\n"]
    pub fn set_notification_channel(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_NotificationChannel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_channel = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_channel = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Output>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regions_of_interest`.\n"]
    pub fn set_regions_of_interest(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_RegionsOfInterest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().regions_of_interest = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.regions_of_interest = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RekognitionStreamProcessor_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nThe identifier for your AWS Key Management Service key (AWS KMS key). You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN."]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn identifier you assign to the stream processor."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor."]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_processor_arn` after provisioning.\n"]
    pub fn stream_processor_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_processor_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sharing_preference` after provisioning.\n"]
    pub fn data_sharing_preference(&self) -> terrars::ListRef<RekognitionStreamProcessor_DataSharingPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_sharing_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<RekognitionStreamProcessor_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_channel` after provisioning.\n"]
    pub fn notification_channel(&self) -> terrars::ListRef<RekognitionStreamProcessor_NotificationChannelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> terrars::ListRef<RekognitionStreamProcessor_OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions_of_interest` after provisioning.\n"]
    pub fn regions_of_interest(&self) -> terrars::ListRef<RekognitionStreamProcessor_RegionsOfInterestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions_of_interest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<RekognitionStreamProcessor_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RekognitionStreamProcessor_TimeoutsRef {
        RekognitionStreamProcessor_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for RekognitionStreamProcessor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RekognitionStreamProcessor { }

impl terrars::ToListMappable for RekognitionStreamProcessor {
    type O = terrars::ListRef<RekognitionStreamProcessorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RekognitionStreamProcessor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_rekognition_stream_processor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRekognitionStreamProcessor {
    pub tf_id: std::string::String,
    #[doc = "An identifier you assign to the stream processor."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor."]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildRekognitionStreamProcessor {
    pub fn build(self, stack: &mut terrars::Stack) -> RekognitionStreamProcessor {
        let out = RekognitionStreamProcessor(std::rc::Rc::new(RekognitionStreamProcessor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RekognitionStreamProcessorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                data_sharing_preference: core::default::Default::default(),
                input: core::default::Default::default(),
                notification_channel: core::default::Default::default(),
                output: core::default::Default::default(),
                regions_of_interest: core::default::Default::default(),
                settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RekognitionStreamProcessorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RekognitionStreamProcessorRef {
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

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nThe identifier for your AWS Key Management Service key (AWS KMS key). You can supply the Amazon Resource Name (ARN) of your KMS key, the ID of your KMS key, an alias for your KMS key, or an alias ARN."]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn identifier you assign to the stream processor."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nThe Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor."]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_processor_arn` after provisioning.\n"]
    pub fn stream_processor_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_processor_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_sharing_preference` after provisioning.\n"]
    pub fn data_sharing_preference(&self) -> terrars::ListRef<RekognitionStreamProcessor_DataSharingPreferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_sharing_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<RekognitionStreamProcessor_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_channel` after provisioning.\n"]
    pub fn notification_channel(&self) -> terrars::ListRef<RekognitionStreamProcessor_NotificationChannelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> terrars::ListRef<RekognitionStreamProcessor_OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions_of_interest` after provisioning.\n"]
    pub fn regions_of_interest(&self) -> terrars::ListRef<RekognitionStreamProcessor_RegionsOfInterestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions_of_interest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<RekognitionStreamProcessor_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RekognitionStreamProcessor_TimeoutsRef {
        RekognitionStreamProcessor_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessorDynamic {
    data_sharing_preference: std
    ::option
    ::Option<terrars::DynamicBlock<RekognitionStreamProcessor_DataSharingPreference>>,
    input: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Input>>,
    notification_channel: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_NotificationChannel>>,
    output: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Output>>,
    regions_of_interest: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_RegionsOfInterest>>,
    settings: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Settings>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_DataSharingPreference {
    opt_in: terrars::PrimField<bool>,
}

impl RekognitionStreamProcessor_DataSharingPreference { }

impl terrars::ToListMappable for RekognitionStreamProcessor_DataSharingPreference {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_DataSharingPreference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_DataSharingPreference {
    #[doc = "Do you want to share data with Rekognition to improve model performance."]
    pub opt_in: terrars::PrimField<bool>,
}

impl BuildRekognitionStreamProcessor_DataSharingPreference {
    pub fn build(self) -> RekognitionStreamProcessor_DataSharingPreference {
        RekognitionStreamProcessor_DataSharingPreference { opt_in: self.opt_in }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_DataSharingPreferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_DataSharingPreferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_DataSharingPreferenceRef {
        RekognitionStreamProcessor_DataSharingPreferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_DataSharingPreferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `opt_in` after provisioning.\nDo you want to share data with Rekognition to improve model performance."]
    pub fn opt_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opt_in", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Input_KinesisVideoStream {
    arn: terrars::PrimField<std::string::String>,
}

impl RekognitionStreamProcessor_Input_KinesisVideoStream { }

impl terrars::ToListMappable for RekognitionStreamProcessor_Input_KinesisVideoStream {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Input_KinesisVideoStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Input_KinesisVideoStream {
    #[doc = "ARN of the Kinesis video stream stream that streams the source video."]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildRekognitionStreamProcessor_Input_KinesisVideoStream {
    pub fn build(self) -> RekognitionStreamProcessor_Input_KinesisVideoStream {
        RekognitionStreamProcessor_Input_KinesisVideoStream { arn: self.arn }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_Input_KinesisVideoStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_Input_KinesisVideoStreamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_Input_KinesisVideoStreamRef {
        RekognitionStreamProcessor_Input_KinesisVideoStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_Input_KinesisVideoStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nARN of the Kinesis video stream stream that streams the source video."]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessor_InputDynamic {
    kinesis_video_stream: std
    ::option
    ::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Input_KinesisVideoStream>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Input {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_video_stream: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Input_KinesisVideoStream>>,
    dynamic: RekognitionStreamProcessor_InputDynamic,
}

impl RekognitionStreamProcessor_Input {
    #[doc = "Set the field `kinesis_video_stream`.\n"]
    pub fn set_kinesis_video_stream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Input_KinesisVideoStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_video_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_video_stream = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Input {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Input {}

impl BuildRekognitionStreamProcessor_Input {
    pub fn build(self) -> RekognitionStreamProcessor_Input {
        RekognitionStreamProcessor_Input {
            kinesis_video_stream: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_InputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RekognitionStreamProcessor_InputRef {
        RekognitionStreamProcessor_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_video_stream` after provisioning.\n"]
    pub fn kinesis_video_stream(&self) -> terrars::ListRef<RekognitionStreamProcessor_Input_KinesisVideoStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_video_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_NotificationChannel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RekognitionStreamProcessor_NotificationChannel {
    #[doc =
        "Set the field `sns_topic_arn`.\nThe Amazon Resource Number (ARN) of the Amazon Amazon Simple Notification Service topic to which Amazon Rekognition posts the completion status."]
    pub fn set_sns_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sns_topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_NotificationChannel {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_NotificationChannel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_NotificationChannel {}

impl BuildRekognitionStreamProcessor_NotificationChannel {
    pub fn build(self) -> RekognitionStreamProcessor_NotificationChannel {
        RekognitionStreamProcessor_NotificationChannel { sns_topic_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_NotificationChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_NotificationChannelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_NotificationChannelRef {
        RekognitionStreamProcessor_NotificationChannelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_NotificationChannelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sns_topic_arn` after provisioning.\nThe Amazon Resource Number (ARN) of the Amazon Amazon Simple Notification Service topic to which Amazon Rekognition posts the completion status."]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Output_KinesisDataStream {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RekognitionStreamProcessor_Output_KinesisDataStream {
    #[doc = "Set the field `arn`.\nARN of the output Amazon Kinesis Data Streams stream."]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Output_KinesisDataStream {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Output_KinesisDataStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Output_KinesisDataStream {}

impl BuildRekognitionStreamProcessor_Output_KinesisDataStream {
    pub fn build(self) -> RekognitionStreamProcessor_Output_KinesisDataStream {
        RekognitionStreamProcessor_Output_KinesisDataStream { arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_Output_KinesisDataStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_Output_KinesisDataStreamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_Output_KinesisDataStreamRef {
        RekognitionStreamProcessor_Output_KinesisDataStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_Output_KinesisDataStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nARN of the output Amazon Kinesis Data Streams stream."]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Output_S3Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RekognitionStreamProcessor_Output_S3Destination {
    #[doc =
        "Set the field `bucket`.\nThe name of the Amazon S3 bucket you want to associate with the streaming video project."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_prefix`.\nThe prefix value of the location within the bucket that you want the information to be published to."]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Output_S3Destination {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Output_S3Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Output_S3Destination {}

impl BuildRekognitionStreamProcessor_Output_S3Destination {
    pub fn build(self) -> RekognitionStreamProcessor_Output_S3Destination {
        RekognitionStreamProcessor_Output_S3Destination {
            bucket: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_Output_S3DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_Output_S3DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_Output_S3DestinationRef {
        RekognitionStreamProcessor_Output_S3DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_Output_S3DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nThe name of the Amazon S3 bucket you want to associate with the streaming video project."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_prefix` after provisioning.\nThe prefix value of the location within the bucket that you want the information to be published to."]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessor_OutputDynamic {
    kinesis_data_stream: std
    ::option
    ::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Output_KinesisDataStream>>,
    s3_destination: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Output_S3Destination>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Output {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_data_stream: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Output_KinesisDataStream>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_destination: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Output_S3Destination>>,
    dynamic: RekognitionStreamProcessor_OutputDynamic,
}

impl RekognitionStreamProcessor_Output {
    #[doc = "Set the field `kinesis_data_stream`.\n"]
    pub fn set_kinesis_data_stream(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Output_KinesisDataStream>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_data_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_data_stream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Output_S3Destination>>,
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

impl terrars::ToListMappable for RekognitionStreamProcessor_Output {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Output {}

impl BuildRekognitionStreamProcessor_Output {
    pub fn build(self) -> RekognitionStreamProcessor_Output {
        RekognitionStreamProcessor_Output {
            kinesis_data_stream: core::default::Default::default(),
            s3_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_OutputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RekognitionStreamProcessor_OutputRef {
        RekognitionStreamProcessor_OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_data_stream` after provisioning.\n"]
    pub fn kinesis_data_stream(&self) -> terrars::ListRef<RekognitionStreamProcessor_Output_KinesisDataStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_data_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<RekognitionStreamProcessor_Output_S3DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_RegionsOfInterest_BoundingBox {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    height: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    width: std::option::Option<terrars::PrimField<f64>>,
}

impl RekognitionStreamProcessor_RegionsOfInterest_BoundingBox {
    #[doc = "Set the field `height`.\nHeight of the bounding box as a ratio of the overall image height."]
    pub fn set_height(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `left`.\nLeft coordinate of the bounding box as a ratio of overall image width."]
    pub fn set_left(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.left = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top`.\nTop coordinate of the bounding box as a ratio of overall image height."]
    pub fn set_top(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `width`.\nWidth of the bounding box as a ratio of the overall image width."]
    pub fn set_width(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.width = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_RegionsOfInterest_BoundingBox {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_RegionsOfInterest_BoundingBox>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_RegionsOfInterest_BoundingBox {}

impl BuildRekognitionStreamProcessor_RegionsOfInterest_BoundingBox {
    pub fn build(self) -> RekognitionStreamProcessor_RegionsOfInterest_BoundingBox {
        RekognitionStreamProcessor_RegionsOfInterest_BoundingBox {
            height: core::default::Default::default(),
            left: core::default::Default::default(),
            top: core::default::Default::default(),
            width: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_RegionsOfInterest_BoundingBoxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_RegionsOfInterest_BoundingBoxRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_RegionsOfInterest_BoundingBoxRef {
        RekognitionStreamProcessor_RegionsOfInterest_BoundingBoxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_RegionsOfInterest_BoundingBoxRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `height` after provisioning.\nHeight of the bounding box as a ratio of the overall image height."]
    pub fn height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc =
        "Get a reference to the value of field `left` after provisioning.\nLeft coordinate of the bounding box as a ratio of overall image width."]
    pub fn left(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.left", self.base))
    }

    #[doc =
        "Get a reference to the value of field `top` after provisioning.\nTop coordinate of the bounding box as a ratio of overall image height."]
    pub fn top(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top", self.base))
    }

    #[doc =
        "Get a reference to the value of field `width` after provisioning.\nWidth of the bounding box as a ratio of the overall image width."]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_RegionsOfInterest_Polygon {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y: std::option::Option<terrars::PrimField<f64>>,
}

impl RekognitionStreamProcessor_RegionsOfInterest_Polygon {
    #[doc = "Set the field `x`.\nThe value of the X coordinate for a point on a Polygon."]
    pub fn set_x(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `y`.\nThe value of the Y coordinate for a point on a Polygon."]
    pub fn set_y(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_RegionsOfInterest_Polygon {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_RegionsOfInterest_Polygon>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_RegionsOfInterest_Polygon {}

impl BuildRekognitionStreamProcessor_RegionsOfInterest_Polygon {
    pub fn build(self) -> RekognitionStreamProcessor_RegionsOfInterest_Polygon {
        RekognitionStreamProcessor_RegionsOfInterest_Polygon {
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_RegionsOfInterest_PolygonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_RegionsOfInterest_PolygonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_RegionsOfInterest_PolygonRef {
        RekognitionStreamProcessor_RegionsOfInterest_PolygonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_RegionsOfInterest_PolygonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `x` after provisioning.\nThe value of the X coordinate for a point on a Polygon."]
    pub fn x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }

    #[doc =
        "Get a reference to the value of field `y` after provisioning.\nThe value of the Y coordinate for a point on a Polygon."]
    pub fn y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessor_RegionsOfInterestDynamic {
    bounding_box: std
    ::option
    ::Option<terrars::DynamicBlock<RekognitionStreamProcessor_RegionsOfInterest_BoundingBox>>,
    polygon: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_RegionsOfInterest_Polygon>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_RegionsOfInterest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bounding_box: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_RegionsOfInterest_BoundingBox>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polygon: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_RegionsOfInterest_Polygon>>,
    dynamic: RekognitionStreamProcessor_RegionsOfInterestDynamic,
}

impl RekognitionStreamProcessor_RegionsOfInterest {
    #[doc = "Set the field `bounding_box`.\n"]
    pub fn set_bounding_box(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<RekognitionStreamProcessor_RegionsOfInterest_BoundingBox>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bounding_box = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bounding_box = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `polygon`.\n"]
    pub fn set_polygon(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_RegionsOfInterest_Polygon>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.polygon = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.polygon = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_RegionsOfInterest {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_RegionsOfInterest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_RegionsOfInterest {}

impl BuildRekognitionStreamProcessor_RegionsOfInterest {
    pub fn build(self) -> RekognitionStreamProcessor_RegionsOfInterest {
        RekognitionStreamProcessor_RegionsOfInterest {
            bounding_box: core::default::Default::default(),
            polygon: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_RegionsOfInterestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_RegionsOfInterestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_RegionsOfInterestRef {
        RekognitionStreamProcessor_RegionsOfInterestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_RegionsOfInterestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bounding_box` after provisioning.\n"]
    pub fn bounding_box(&self) -> terrars::ListRef<RekognitionStreamProcessor_RegionsOfInterest_BoundingBoxRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bounding_box", self.base))
    }

    #[doc = "Get a reference to the value of field `polygon` after provisioning.\n"]
    pub fn polygon(&self) -> terrars::ListRef<RekognitionStreamProcessor_RegionsOfInterest_PolygonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.polygon", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Settings_ConnectedHome {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_confidence: std::option::Option<terrars::PrimField<f64>>,
}

impl RekognitionStreamProcessor_Settings_ConnectedHome {
    #[doc =
        "Set the field `labels`.\nSpecifies what you want to detect in the video, such as people, packages, or pets."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_confidence`.\nThe minimum confidence required to label an object in the video."]
    pub fn set_min_confidence(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_confidence = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Settings_ConnectedHome {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Settings_ConnectedHome>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Settings_ConnectedHome {}

impl BuildRekognitionStreamProcessor_Settings_ConnectedHome {
    pub fn build(self) -> RekognitionStreamProcessor_Settings_ConnectedHome {
        RekognitionStreamProcessor_Settings_ConnectedHome {
            labels: core::default::Default::default(),
            min_confidence: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_Settings_ConnectedHomeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_Settings_ConnectedHomeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_Settings_ConnectedHomeRef {
        RekognitionStreamProcessor_Settings_ConnectedHomeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_Settings_ConnectedHomeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSpecifies what you want to detect in the video, such as people, packages, or pets."]
    pub fn labels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_confidence` after provisioning.\nThe minimum confidence required to label an object in the video."]
    pub fn min_confidence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_confidence", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Settings_FaceSearch {
    collection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    face_match_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl RekognitionStreamProcessor_Settings_FaceSearch {
    #[doc =
        "Set the field `face_match_threshold`.\nMinimum face match confidence score that must be met to return a result for a recognized face."]
    pub fn set_face_match_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.face_match_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Settings_FaceSearch {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Settings_FaceSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Settings_FaceSearch {
    #[doc = "The ID of a collection that contains faces that you want to search for."]
    pub collection_id: terrars::PrimField<std::string::String>,
}

impl BuildRekognitionStreamProcessor_Settings_FaceSearch {
    pub fn build(self) -> RekognitionStreamProcessor_Settings_FaceSearch {
        RekognitionStreamProcessor_Settings_FaceSearch {
            collection_id: self.collection_id,
            face_match_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_Settings_FaceSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_Settings_FaceSearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RekognitionStreamProcessor_Settings_FaceSearchRef {
        RekognitionStreamProcessor_Settings_FaceSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_Settings_FaceSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe ID of a collection that contains faces that you want to search for."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `face_match_threshold` after provisioning.\nMinimum face match confidence score that must be met to return a result for a recognized face."]
    pub fn face_match_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.face_match_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct RekognitionStreamProcessor_SettingsDynamic {
    connected_home: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Settings_ConnectedHome>>,
    face_search: std::option::Option<terrars::DynamicBlock<RekognitionStreamProcessor_Settings_FaceSearch>>,
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connected_home: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Settings_ConnectedHome>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    face_search: std::option::Option<std::vec::Vec<RekognitionStreamProcessor_Settings_FaceSearch>>,
    dynamic: RekognitionStreamProcessor_SettingsDynamic,
}

impl RekognitionStreamProcessor_Settings {
    #[doc = "Set the field `connected_home`.\n"]
    pub fn set_connected_home(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Settings_ConnectedHome>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connected_home = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connected_home = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `face_search`.\n"]
    pub fn set_face_search(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RekognitionStreamProcessor_Settings_FaceSearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.face_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.face_search = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Settings {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Settings {}

impl BuildRekognitionStreamProcessor_Settings {
    pub fn build(self) -> RekognitionStreamProcessor_Settings {
        RekognitionStreamProcessor_Settings {
            connected_home: core::default::Default::default(),
            face_search: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RekognitionStreamProcessor_SettingsRef {
        RekognitionStreamProcessor_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connected_home` after provisioning.\n"]
    pub fn connected_home(&self) -> terrars::ListRef<RekognitionStreamProcessor_Settings_ConnectedHomeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connected_home", self.base))
    }

    #[doc = "Get a reference to the value of field `face_search` after provisioning.\n"]
    pub fn face_search(&self) -> terrars::ListRef<RekognitionStreamProcessor_Settings_FaceSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.face_search", self.base))
    }
}

#[derive(Serialize)]
pub struct RekognitionStreamProcessor_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RekognitionStreamProcessor_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RekognitionStreamProcessor_Timeouts {
    type O = terrars::BlockAssignable<RekognitionStreamProcessor_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRekognitionStreamProcessor_Timeouts {}

impl BuildRekognitionStreamProcessor_Timeouts {
    pub fn build(self) -> RekognitionStreamProcessor_Timeouts {
        RekognitionStreamProcessor_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RekognitionStreamProcessor_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RekognitionStreamProcessor_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RekognitionStreamProcessor_TimeoutsRef {
        RekognitionStreamProcessor_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RekognitionStreamProcessor_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
