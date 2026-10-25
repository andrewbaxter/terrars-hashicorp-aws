#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppstreamStackData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embed_host_domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feedback_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_endpoints: std::option::Option<std::vec::Vec<AppstreamStack_AccessEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_settings: std::option::Option<std::vec::Vec<AppstreamStack_ApplicationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_connectors: std::option::Option<std::vec::Vec<AppstreamStack_StorageConnectors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    streaming_experience_settings: std::option::Option<std::vec::Vec<AppstreamStack_StreamingExperienceSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_settings: std::option::Option<std::vec::Vec<AppstreamStack_UserSettings>>,
    dynamic: AppstreamStackDynamic,
}

struct AppstreamStack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppstreamStackData_>,
}

#[derive(Clone)]
pub struct AppstreamStack(std::rc::Rc<AppstreamStack_>);

impl AppstreamStack {
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

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embed_host_domains`.\n"]
    pub fn set_embed_host_domains(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().embed_host_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feedback_url`.\n"]
    pub fn set_feedback_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().feedback_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_url`.\n"]
    pub fn set_redirect_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().redirect_url = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_endpoints`.\n"]
    pub fn set_access_endpoints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppstreamStack_AccessEndpoints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_endpoints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `application_settings`.\n"]
    pub fn set_application_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppstreamStack_ApplicationSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_connectors`.\n"]
    pub fn set_storage_connectors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppstreamStack_StorageConnectors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_connectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_connectors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `streaming_experience_settings`.\n"]
    pub fn set_streaming_experience_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppstreamStack_StreamingExperienceSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().streaming_experience_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.streaming_experience_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_settings`.\n"]
    pub fn set_user_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppstreamStack_UserSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `embed_host_domains` after provisioning.\n"]
    pub fn embed_host_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.embed_host_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feedback_url` after provisioning.\n"]
    pub fn feedback_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feedback_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> terrars::ListRef<AppstreamStack_ApplicationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `streaming_experience_settings` after provisioning.\n"]
    pub fn streaming_experience_settings(&self) -> terrars::ListRef<AppstreamStack_StreamingExperienceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.streaming_experience_settings", self.extract_ref()))
    }
}

impl terrars::Referable for AppstreamStack {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppstreamStack { }

impl terrars::ToListMappable for AppstreamStack {
    type O = terrars::ListRef<AppstreamStackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppstreamStack_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appstream_stack".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppstreamStack {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppstreamStack {
    pub fn build(self, stack: &mut terrars::Stack) -> AppstreamStack {
        let out = AppstreamStack(std::rc::Rc::new(AppstreamStack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppstreamStackData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                embed_host_domains: core::default::Default::default(),
                feedback_url: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                redirect_url: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_endpoints: core::default::Default::default(),
                application_settings: core::default::Default::default(),
                storage_connectors: core::default::Default::default(),
                streaming_experience_settings: core::default::Default::default(),
                user_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppstreamStackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamStackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppstreamStackRef {
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

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `embed_host_domains` after provisioning.\n"]
    pub fn embed_host_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.embed_host_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feedback_url` after provisioning.\n"]
    pub fn feedback_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feedback_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> terrars::ListRef<AppstreamStack_ApplicationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `streaming_experience_settings` after provisioning.\n"]
    pub fn streaming_experience_settings(&self) -> terrars::ListRef<AppstreamStack_StreamingExperienceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.streaming_experience_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppstreamStackDynamic {
    access_endpoints: std::option::Option<terrars::DynamicBlock<AppstreamStack_AccessEndpoints>>,
    application_settings: std::option::Option<terrars::DynamicBlock<AppstreamStack_ApplicationSettings>>,
    storage_connectors: std::option::Option<terrars::DynamicBlock<AppstreamStack_StorageConnectors>>,
    streaming_experience_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AppstreamStack_StreamingExperienceSettings>>,
    user_settings: std::option::Option<terrars::DynamicBlock<AppstreamStack_UserSettings>>,
}

#[derive(Serialize)]
pub struct AppstreamStack_AccessEndpoints {
    endpoint_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpce_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppstreamStack_AccessEndpoints {
    #[doc = "Set the field `vpce_id`.\n"]
    pub fn set_vpce_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpce_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppstreamStack_AccessEndpoints {
    type O = terrars::BlockAssignable<AppstreamStack_AccessEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStack_AccessEndpoints {
    #[doc = ""]
    pub endpoint_type: terrars::PrimField<std::string::String>,
}

impl BuildAppstreamStack_AccessEndpoints {
    pub fn build(self) -> AppstreamStack_AccessEndpoints {
        AppstreamStack_AccessEndpoints {
            endpoint_type: self.endpoint_type,
            vpce_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppstreamStack_AccessEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamStack_AccessEndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppstreamStack_AccessEndpointsRef {
        AppstreamStack_AccessEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStack_AccessEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vpce_id` after provisioning.\n"]
    pub fn vpce_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpce_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStack_ApplicationSettings {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppstreamStack_ApplicationSettings {
    #[doc = "Set the field `settings_group`.\n"]
    pub fn set_settings_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.settings_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppstreamStack_ApplicationSettings {
    type O = terrars::BlockAssignable<AppstreamStack_ApplicationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStack_ApplicationSettings {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAppstreamStack_ApplicationSettings {
    pub fn build(self) -> AppstreamStack_ApplicationSettings {
        AppstreamStack_ApplicationSettings {
            enabled: self.enabled,
            settings_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppstreamStack_ApplicationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamStack_ApplicationSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppstreamStack_ApplicationSettingsRef {
        AppstreamStack_ApplicationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStack_ApplicationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `settings_group` after provisioning.\n"]
    pub fn settings_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_group", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStack_StorageConnectors {
    connector_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppstreamStack_StorageConnectors {
    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_identifier`.\n"]
    pub fn set_resource_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppstreamStack_StorageConnectors {
    type O = terrars::BlockAssignable<AppstreamStack_StorageConnectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStack_StorageConnectors {
    #[doc = ""]
    pub connector_type: terrars::PrimField<std::string::String>,
}

impl BuildAppstreamStack_StorageConnectors {
    pub fn build(self) -> AppstreamStack_StorageConnectors {
        AppstreamStack_StorageConnectors {
            connector_type: self.connector_type,
            domains: core::default::Default::default(),
            resource_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppstreamStack_StorageConnectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamStack_StorageConnectorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppstreamStack_StorageConnectorsRef {
        AppstreamStack_StorageConnectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStack_StorageConnectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStack_StreamingExperienceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppstreamStack_StreamingExperienceSettings {
    #[doc = "Set the field `preferred_protocol`.\n"]
    pub fn set_preferred_protocol(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preferred_protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppstreamStack_StreamingExperienceSettings {
    type O = terrars::BlockAssignable<AppstreamStack_StreamingExperienceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStack_StreamingExperienceSettings {}

impl BuildAppstreamStack_StreamingExperienceSettings {
    pub fn build(self) -> AppstreamStack_StreamingExperienceSettings {
        AppstreamStack_StreamingExperienceSettings { preferred_protocol: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AppstreamStack_StreamingExperienceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamStack_StreamingExperienceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppstreamStack_StreamingExperienceSettingsRef {
        AppstreamStack_StreamingExperienceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStack_StreamingExperienceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preferred_protocol` after provisioning.\n"]
    pub fn preferred_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStack_UserSettings {
    action: terrars::PrimField<std::string::String>,
    permission: terrars::PrimField<std::string::String>,
}

impl AppstreamStack_UserSettings { }

impl terrars::ToListMappable for AppstreamStack_UserSettings {
    type O = terrars::BlockAssignable<AppstreamStack_UserSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStack_UserSettings {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission: terrars::PrimField<std::string::String>,
}

impl BuildAppstreamStack_UserSettings {
    pub fn build(self) -> AppstreamStack_UserSettings {
        AppstreamStack_UserSettings {
            action: self.action,
            permission: self.permission,
        }
    }
}

#[allow(dead_code)]
pub struct AppstreamStack_UserSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamStack_UserSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppstreamStack_UserSettingsRef {
        AppstreamStack_UserSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStack_UserSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}
