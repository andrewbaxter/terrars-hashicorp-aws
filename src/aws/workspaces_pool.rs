#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspacesPoolData_ {
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
    application_settings: std::option::Option<terrars::ListField<WorkspacesPool_ApplicationSettings>>,
    bundle_id: terrars::PrimField<std::string::String>,
    description: terrars::PrimField<std::string::String>,
    directory_id: terrars::PrimField<std::string::String>,
    pool_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    running_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_settings: std::option::Option<terrars::ListField<WorkspacesPool_TimeoutSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity: std::option::Option<std::vec::Vec<WorkspacesPool_Capacity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<WorkspacesPool_Timeouts>,
    dynamic: WorkspacesPoolDynamic,
}

struct WorkspacesPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkspacesPoolData_>,
}

#[derive(Clone)]
pub struct WorkspacesPool(std::rc::Rc<WorkspacesPool_>);

impl WorkspacesPool {
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

    #[doc = "Set the field `application_settings`.\n"]
    pub fn set_application_settings(
        self,
        v: impl std::convert::Into<terrars::ListField<WorkspacesPool_ApplicationSettings>>,
    ) -> Self {
        self.0.data.borrow_mut().application_settings = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeout_settings`.\n"]
    pub fn set_timeout_settings(
        self,
        v: impl std::convert::Into<terrars::ListField<WorkspacesPool_TimeoutSettings>>,
    ) -> Self {
        self.0.data.borrow_mut().timeout_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(self, v: impl std::convert::Into<terrars::BlockAssignable<WorkspacesPool_Capacity>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<WorkspacesPool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> terrars::ListRef<WorkspacesPool_ApplicationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_status` after provisioning.\n"]
    pub fn capacity_status(&self) -> terrars::ListRef<WorkspacesPool_CapacityStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_arn` after provisioning.\n"]
    pub fn pool_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\n"]
    pub fn pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `running_mode` after provisioning.\n"]
    pub fn running_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeout_settings` after provisioning.\n"]
    pub fn timeout_settings(&self) -> terrars::ListRef<WorkspacesPool_TimeoutSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<WorkspacesPool_CapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkspacesPool_TimeoutsRef {
        WorkspacesPool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for WorkspacesPool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkspacesPool { }

impl terrars::ToListMappable for WorkspacesPool {
    type O = terrars::ListRef<WorkspacesPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkspacesPool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_workspaces_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspacesPool {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bundle_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub directory_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub pool_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub running_mode: terrars::PrimField<std::string::String>,
}

impl BuildWorkspacesPool {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkspacesPool {
        let out = WorkspacesPool(std::rc::Rc::new(WorkspacesPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkspacesPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_settings: core::default::Default::default(),
                bundle_id: self.bundle_id,
                description: self.description,
                directory_id: self.directory_id,
                pool_name: self.pool_name,
                region: core::default::Default::default(),
                running_mode: self.running_mode,
                tags: core::default::Default::default(),
                timeout_settings: core::default::Default::default(),
                capacity: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspacesPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkspacesPoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> terrars::ListRef<WorkspacesPool_ApplicationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_status` after provisioning.\n"]
    pub fn capacity_status(&self) -> terrars::ListRef<WorkspacesPool_CapacityStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_arn` after provisioning.\n"]
    pub fn pool_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\n"]
    pub fn pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `running_mode` after provisioning.\n"]
    pub fn running_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeout_settings` after provisioning.\n"]
    pub fn timeout_settings(&self) -> terrars::ListRef<WorkspacesPool_TimeoutSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<WorkspacesPool_CapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkspacesPool_TimeoutsRef {
        WorkspacesPool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WorkspacesPoolDynamic {
    capacity: std::option::Option<terrars::DynamicBlock<WorkspacesPool_Capacity>>,
}

#[derive(Serialize)]
pub struct WorkspacesPool_ApplicationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkspacesPool_ApplicationSettings {
    #[doc = "Set the field `settings_group`.\n"]
    pub fn set_settings_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.settings_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspacesPool_ApplicationSettings {
    type O = terrars::BlockAssignable<WorkspacesPool_ApplicationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesPool_ApplicationSettings {}

impl BuildWorkspacesPool_ApplicationSettings {
    pub fn build(self) -> WorkspacesPool_ApplicationSettings {
        WorkspacesPool_ApplicationSettings {
            settings_group: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspacesPool_ApplicationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesPool_ApplicationSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspacesPool_ApplicationSettingsRef {
        WorkspacesPool_ApplicationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesPool_ApplicationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `settings_group` after provisioning.\n"]
    pub fn settings_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_group", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspacesPool_CapacityStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_user_sessions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actual_user_sessions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_user_sessions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_user_sessions: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkspacesPool_CapacityStatus {
    #[doc = "Set the field `active_user_sessions`.\n"]
    pub fn set_active_user_sessions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.active_user_sessions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actual_user_sessions`.\n"]
    pub fn set_actual_user_sessions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.actual_user_sessions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_user_sessions`.\n"]
    pub fn set_available_user_sessions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_user_sessions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_user_sessions`.\n"]
    pub fn set_desired_user_sessions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_user_sessions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspacesPool_CapacityStatus {
    type O = terrars::BlockAssignable<WorkspacesPool_CapacityStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesPool_CapacityStatus {}

impl BuildWorkspacesPool_CapacityStatus {
    pub fn build(self) -> WorkspacesPool_CapacityStatus {
        WorkspacesPool_CapacityStatus {
            active_user_sessions: core::default::Default::default(),
            actual_user_sessions: core::default::Default::default(),
            available_user_sessions: core::default::Default::default(),
            desired_user_sessions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspacesPool_CapacityStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesPool_CapacityStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspacesPool_CapacityStatusRef {
        WorkspacesPool_CapacityStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesPool_CapacityStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_user_sessions` after provisioning.\n"]
    pub fn active_user_sessions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_user_sessions", self.base))
    }

    #[doc = "Get a reference to the value of field `actual_user_sessions` after provisioning.\n"]
    pub fn actual_user_sessions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actual_user_sessions", self.base))
    }

    #[doc = "Get a reference to the value of field `available_user_sessions` after provisioning.\n"]
    pub fn available_user_sessions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_user_sessions", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_user_sessions` after provisioning.\n"]
    pub fn desired_user_sessions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_user_sessions", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspacesPool_TimeoutSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disconnect_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_disconnect_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_user_duration_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkspacesPool_TimeoutSettings {
    #[doc = "Set the field `disconnect_timeout_in_seconds`.\n"]
    pub fn set_disconnect_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disconnect_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_disconnect_timeout_in_seconds`.\n"]
    pub fn set_idle_disconnect_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.idle_disconnect_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_user_duration_in_seconds`.\n"]
    pub fn set_max_user_duration_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_user_duration_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspacesPool_TimeoutSettings {
    type O = terrars::BlockAssignable<WorkspacesPool_TimeoutSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesPool_TimeoutSettings {}

impl BuildWorkspacesPool_TimeoutSettings {
    pub fn build(self) -> WorkspacesPool_TimeoutSettings {
        WorkspacesPool_TimeoutSettings {
            disconnect_timeout_in_seconds: core::default::Default::default(),
            idle_disconnect_timeout_in_seconds: core::default::Default::default(),
            max_user_duration_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspacesPool_TimeoutSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesPool_TimeoutSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspacesPool_TimeoutSettingsRef {
        WorkspacesPool_TimeoutSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesPool_TimeoutSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_seconds` after provisioning.\n"]
    pub fn disconnect_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disconnect_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_seconds` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_disconnect_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `max_user_duration_in_seconds` after provisioning.\n"]
    pub fn max_user_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_user_duration_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspacesPool_Capacity {
    desired_user_sessions: terrars::PrimField<f64>,
}

impl WorkspacesPool_Capacity { }

impl terrars::ToListMappable for WorkspacesPool_Capacity {
    type O = terrars::BlockAssignable<WorkspacesPool_Capacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesPool_Capacity {
    #[doc = ""]
    pub desired_user_sessions: terrars::PrimField<f64>,
}

impl BuildWorkspacesPool_Capacity {
    pub fn build(self) -> WorkspacesPool_Capacity {
        WorkspacesPool_Capacity { desired_user_sessions: self.desired_user_sessions }
    }
}

#[allow(dead_code)]
pub struct WorkspacesPool_CapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesPool_CapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspacesPool_CapacityRef {
        WorkspacesPool_CapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesPool_CapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_user_sessions` after provisioning.\n"]
    pub fn desired_user_sessions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_user_sessions", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspacesPool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkspacesPool_Timeouts {
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

impl terrars::ToListMappable for WorkspacesPool_Timeouts {
    type O = terrars::BlockAssignable<WorkspacesPool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesPool_Timeouts {}

impl BuildWorkspacesPool_Timeouts {
    pub fn build(self) -> WorkspacesPool_Timeouts {
        WorkspacesPool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspacesPool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesPool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspacesPool_TimeoutsRef {
        WorkspacesPool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesPool_TimeoutsRef {
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
