#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudtrailData_ {
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
    cloud_watch_logs_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_logs_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_log_file_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_global_service_events: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_multi_region_trail: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_organization_trail: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_topic_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_event_selector: std::option::Option<std::vec::Vec<Cloudtrail_AdvancedEventSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_selector: std::option::Option<std::vec::Vec<Cloudtrail_EventSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insight_selector: std::option::Option<std::vec::Vec<Cloudtrail_InsightSelector>>,
    dynamic: CloudtrailDynamic,
}

struct Cloudtrail_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudtrailData_>,
}

#[derive(Clone)]
pub struct Cloudtrail(std::rc::Rc<Cloudtrail_>);

impl Cloudtrail {
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

    #[doc = "Set the field `cloud_watch_logs_group_arn`.\n"]
    pub fn set_cloud_watch_logs_group_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloud_watch_logs_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_logs_role_arn`.\n"]
    pub fn set_cloud_watch_logs_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloud_watch_logs_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_log_file_validation`.\n"]
    pub fn set_enable_log_file_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_log_file_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_logging`.\n"]
    pub fn set_enable_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_global_service_events`.\n"]
    pub fn set_include_global_service_events(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_global_service_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_multi_region_trail`.\n"]
    pub fn set_is_multi_region_trail(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_multi_region_trail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_organization_trail`.\n"]
    pub fn set_is_organization_trail(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_organization_trail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
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

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_key_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sns_topic_name`.\n"]
    pub fn set_sns_topic_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sns_topic_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `advanced_event_selector`.\n"]
    pub fn set_advanced_event_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudtrail_AdvancedEventSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_event_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_event_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_selector`.\n"]
    pub fn set_event_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudtrail_EventSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `insight_selector`.\n"]
    pub fn set_insight_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudtrail_InsightSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().insight_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.insight_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_group_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_role_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_log_file_validation` after provisioning.\n"]
    pub fn enable_log_file_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_log_file_validation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_global_service_events` after provisioning.\n"]
    pub fn include_global_service_events(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_global_service_events", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_multi_region_trail` after provisioning.\n"]
    pub fn is_multi_region_trail(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_multi_region_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_organization_trail` after provisioning.\n"]
    pub fn is_organization_trail(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_organization_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_name` after provisioning.\n"]
    pub fn sns_topic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> terrars::ListRef<Cloudtrail_AdvancedEventSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_selector` after provisioning.\n"]
    pub fn event_selector(&self) -> terrars::ListRef<Cloudtrail_EventSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_selector", self.extract_ref()))
    }
}

impl terrars::Referable for Cloudtrail {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Cloudtrail { }

impl terrars::ToListMappable for Cloudtrail {
    type O = terrars::ListRef<CloudtrailRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Cloudtrail_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudtrail".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudtrail {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudtrail {
    pub fn build(self, stack: &mut terrars::Stack) -> Cloudtrail {
        let out = Cloudtrail(std::rc::Rc::new(Cloudtrail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudtrailData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cloud_watch_logs_group_arn: core::default::Default::default(),
                cloud_watch_logs_role_arn: core::default::Default::default(),
                enable_log_file_validation: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                include_global_service_events: core::default::Default::default(),
                is_multi_region_trail: core::default::Default::default(),
                is_organization_trail: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                s3_bucket_name: self.s3_bucket_name,
                s3_key_prefix: core::default::Default::default(),
                sns_topic_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_event_selector: core::default::Default::default(),
                event_selector: core::default::Default::default(),
                insight_selector: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudtrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudtrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudtrailRef {
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

    #[doc = "Get a reference to the value of field `cloud_watch_logs_group_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_role_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_log_file_validation` after provisioning.\n"]
    pub fn enable_log_file_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_log_file_validation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_global_service_events` after provisioning.\n"]
    pub fn include_global_service_events(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_global_service_events", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_multi_region_trail` after provisioning.\n"]
    pub fn is_multi_region_trail(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_multi_region_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_organization_trail` after provisioning.\n"]
    pub fn is_organization_trail(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_organization_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_name` after provisioning.\n"]
    pub fn sns_topic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> terrars::ListRef<Cloudtrail_AdvancedEventSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_selector` after provisioning.\n"]
    pub fn event_selector(&self) -> terrars::ListRef<Cloudtrail_EventSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_selector", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailDynamic {
    advanced_event_selector: std::option::Option<terrars::DynamicBlock<Cloudtrail_AdvancedEventSelector>>,
    event_selector: std::option::Option<terrars::DynamicBlock<Cloudtrail_EventSelector>>,
    insight_selector: std::option::Option<terrars::DynamicBlock<Cloudtrail_InsightSelector>>,
}

#[derive(Serialize)]
pub struct Cloudtrail_AdvancedEventSelector_FieldSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ends_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_ends_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_starts_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starts_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Cloudtrail_AdvancedEventSelector_FieldSelector {
    #[doc = "Set the field `ends_with`.\n"]
    pub fn set_ends_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `equals`.\n"]
    pub fn set_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_ends_with`.\n"]
    pub fn set_not_ends_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_equals`.\n"]
    pub fn set_not_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_starts_with`.\n"]
    pub fn set_not_starts_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_starts_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starts_with`.\n"]
    pub fn set_starts_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.starts_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudtrail_AdvancedEventSelector_FieldSelector {
    type O = terrars::BlockAssignable<Cloudtrail_AdvancedEventSelector_FieldSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrail_AdvancedEventSelector_FieldSelector {
    #[doc = ""]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildCloudtrail_AdvancedEventSelector_FieldSelector {
    pub fn build(self) -> Cloudtrail_AdvancedEventSelector_FieldSelector {
        Cloudtrail_AdvancedEventSelector_FieldSelector {
            ends_with: core::default::Default::default(),
            equals: core::default::Default::default(),
            field: self.field,
            not_ends_with: core::default::Default::default(),
            not_equals: core::default::Default::default(),
            not_starts_with: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudtrail_AdvancedEventSelector_FieldSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudtrail_AdvancedEventSelector_FieldSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudtrail_AdvancedEventSelector_FieldSelectorRef {
        Cloudtrail_AdvancedEventSelector_FieldSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudtrail_AdvancedEventSelector_FieldSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ends_with` after provisioning.\n"]
    pub fn ends_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `equals` after provisioning.\n"]
    pub fn equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.equals", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `not_ends_with` after provisioning.\n"]
    pub fn not_ends_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `not_equals` after provisioning.\n"]
    pub fn not_equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `not_starts_with` after provisioning.\n"]
    pub fn not_starts_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_starts_with", self.base))
    }

    #[doc = "Get a reference to the value of field `starts_with` after provisioning.\n"]
    pub fn starts_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.starts_with", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudtrail_AdvancedEventSelectorDynamic {
    field_selector: std::option::Option<terrars::DynamicBlock<Cloudtrail_AdvancedEventSelector_FieldSelector>>,
}

#[derive(Serialize)]
pub struct Cloudtrail_AdvancedEventSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_selector: std::option::Option<std::vec::Vec<Cloudtrail_AdvancedEventSelector_FieldSelector>>,
    dynamic: Cloudtrail_AdvancedEventSelectorDynamic,
}

impl Cloudtrail_AdvancedEventSelector {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_selector`.\n"]
    pub fn set_field_selector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudtrail_AdvancedEventSelector_FieldSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudtrail_AdvancedEventSelector {
    type O = terrars::BlockAssignable<Cloudtrail_AdvancedEventSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrail_AdvancedEventSelector {}

impl BuildCloudtrail_AdvancedEventSelector {
    pub fn build(self) -> Cloudtrail_AdvancedEventSelector {
        Cloudtrail_AdvancedEventSelector {
            name: core::default::Default::default(),
            field_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudtrail_AdvancedEventSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudtrail_AdvancedEventSelectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudtrail_AdvancedEventSelectorRef {
        Cloudtrail_AdvancedEventSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudtrail_AdvancedEventSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudtrail_EventSelector_DataResource {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Cloudtrail_EventSelector_DataResource { }

impl terrars::ToListMappable for Cloudtrail_EventSelector_DataResource {
    type O = terrars::BlockAssignable<Cloudtrail_EventSelector_DataResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrail_EventSelector_DataResource {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudtrail_EventSelector_DataResource {
    pub fn build(self) -> Cloudtrail_EventSelector_DataResource {
        Cloudtrail_EventSelector_DataResource {
            type_: self.type_,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct Cloudtrail_EventSelector_DataResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudtrail_EventSelector_DataResourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudtrail_EventSelector_DataResourceRef {
        Cloudtrail_EventSelector_DataResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudtrail_EventSelector_DataResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudtrail_EventSelectorDynamic {
    data_resource: std::option::Option<terrars::DynamicBlock<Cloudtrail_EventSelector_DataResource>>,
}

#[derive(Serialize)]
pub struct Cloudtrail_EventSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_management_event_sources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_management_events: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_write_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_resource: std::option::Option<std::vec::Vec<Cloudtrail_EventSelector_DataResource>>,
    dynamic: Cloudtrail_EventSelectorDynamic,
}

impl Cloudtrail_EventSelector {
    #[doc = "Set the field `exclude_management_event_sources`.\n"]
    pub fn set_exclude_management_event_sources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_management_event_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_management_events`.\n"]
    pub fn set_include_management_events(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_management_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_write_type`.\n"]
    pub fn set_read_write_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read_write_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_resource`.\n"]
    pub fn set_data_resource(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudtrail_EventSelector_DataResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudtrail_EventSelector {
    type O = terrars::BlockAssignable<Cloudtrail_EventSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrail_EventSelector {}

impl BuildCloudtrail_EventSelector {
    pub fn build(self) -> Cloudtrail_EventSelector {
        Cloudtrail_EventSelector {
            exclude_management_event_sources: core::default::Default::default(),
            include_management_events: core::default::Default::default(),
            read_write_type: core::default::Default::default(),
            data_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudtrail_EventSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudtrail_EventSelectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudtrail_EventSelectorRef {
        Cloudtrail_EventSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudtrail_EventSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_management_event_sources` after provisioning.\n"]
    pub fn exclude_management_event_sources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude_management_event_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `include_management_events` after provisioning.\n"]
    pub fn include_management_events(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_management_events", self.base))
    }

    #[doc = "Get a reference to the value of field `read_write_type` after provisioning.\n"]
    pub fn read_write_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_write_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_resource` after provisioning.\n"]
    pub fn data_resource(&self) -> terrars::ListRef<Cloudtrail_EventSelector_DataResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudtrail_InsightSelector {
    insight_type: terrars::PrimField<std::string::String>,
}

impl Cloudtrail_InsightSelector { }

impl terrars::ToListMappable for Cloudtrail_InsightSelector {
    type O = terrars::BlockAssignable<Cloudtrail_InsightSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrail_InsightSelector {
    #[doc = ""]
    pub insight_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudtrail_InsightSelector {
    pub fn build(self) -> Cloudtrail_InsightSelector {
        Cloudtrail_InsightSelector { insight_type: self.insight_type }
    }
}

#[allow(dead_code)]
pub struct Cloudtrail_InsightSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudtrail_InsightSelectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudtrail_InsightSelectorRef {
        Cloudtrail_InsightSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudtrail_InsightSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insight_type` after provisioning.\n"]
    pub fn insight_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insight_type", self.base))
    }
}
