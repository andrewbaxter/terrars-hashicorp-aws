#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspaceswebSessionLoggerData_ {
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
    additional_encryption_context: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_filter: std::option::Option<std::vec::Vec<WorkspaceswebSessionLogger_EventFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<WorkspaceswebSessionLogger_LogConfiguration>>,
    dynamic: WorkspaceswebSessionLoggerDynamic,
}

struct WorkspaceswebSessionLogger_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkspaceswebSessionLoggerData_>,
}

#[derive(Clone)]
pub struct WorkspaceswebSessionLogger(std::rc::Rc<WorkspaceswebSessionLogger_>);

impl WorkspaceswebSessionLogger {
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

    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `event_filter`.\n"]
    pub fn set_event_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspaceswebSessionLogger_EventFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspaceswebSessionLogger_LogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_logger_arn` after provisioning.\n"]
    pub fn session_logger_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_logger_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_filter` after provisioning.\n"]
    pub fn event_filter(&self) -> terrars::ListRef<WorkspaceswebSessionLogger_EventFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<WorkspaceswebSessionLogger_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for WorkspaceswebSessionLogger {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkspaceswebSessionLogger { }

impl terrars::ToListMappable for WorkspaceswebSessionLogger {
    type O = terrars::ListRef<WorkspaceswebSessionLoggerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkspaceswebSessionLogger_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_workspacesweb_session_logger".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspaceswebSessionLogger {
    pub tf_id: std::string::String,
}

impl BuildWorkspaceswebSessionLogger {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkspaceswebSessionLogger {
        let out = WorkspaceswebSessionLogger(std::rc::Rc::new(WorkspaceswebSessionLogger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkspaceswebSessionLoggerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                additional_encryption_context: core::default::Default::default(),
                customer_managed_key: core::default::Default::default(),
                display_name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                event_filter: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspaceswebSessionLoggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebSessionLoggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkspaceswebSessionLoggerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_logger_arn` after provisioning.\n"]
    pub fn session_logger_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_logger_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_filter` after provisioning.\n"]
    pub fn event_filter(&self) -> terrars::ListRef<WorkspaceswebSessionLogger_EventFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<WorkspaceswebSessionLogger_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebSessionLoggerDynamic {
    event_filter: std::option::Option<terrars::DynamicBlock<WorkspaceswebSessionLogger_EventFilter>>,
    log_configuration: std::option::Option<terrars::DynamicBlock<WorkspaceswebSessionLogger_LogConfiguration>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebSessionLogger_EventFilter_All {}

impl WorkspaceswebSessionLogger_EventFilter_All { }

impl terrars::ToListMappable for WorkspaceswebSessionLogger_EventFilter_All {
    type O = terrars::BlockAssignable<WorkspaceswebSessionLogger_EventFilter_All>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebSessionLogger_EventFilter_All {}

impl BuildWorkspaceswebSessionLogger_EventFilter_All {
    pub fn build(self) -> WorkspaceswebSessionLogger_EventFilter_All {
        WorkspaceswebSessionLogger_EventFilter_All {}
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebSessionLogger_EventFilter_AllRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebSessionLogger_EventFilter_AllRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspaceswebSessionLogger_EventFilter_AllRef {
        WorkspaceswebSessionLogger_EventFilter_AllRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebSessionLogger_EventFilter_AllRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebSessionLogger_EventFilterDynamic {
    all: std::option::Option<terrars::DynamicBlock<WorkspaceswebSessionLogger_EventFilter_All>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebSessionLogger_EventFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<std::vec::Vec<WorkspaceswebSessionLogger_EventFilter_All>>,
    dynamic: WorkspaceswebSessionLogger_EventFilterDynamic,
}

impl WorkspaceswebSessionLogger_EventFilter {
    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all`.\n"]
    pub fn set_all(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspaceswebSessionLogger_EventFilter_All>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebSessionLogger_EventFilter {
    type O = terrars::BlockAssignable<WorkspaceswebSessionLogger_EventFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebSessionLogger_EventFilter {}

impl BuildWorkspaceswebSessionLogger_EventFilter {
    pub fn build(self) -> WorkspaceswebSessionLogger_EventFilter {
        WorkspaceswebSessionLogger_EventFilter {
            include: core::default::Default::default(),
            all: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebSessionLogger_EventFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebSessionLogger_EventFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkspaceswebSessionLogger_EventFilterRef {
        WorkspaceswebSessionLogger_EventFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebSessionLogger_EventFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> terrars::ListRef<WorkspaceswebSessionLogger_EventFilter_AllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebSessionLogger_LogConfiguration_S3 {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    folder_structure: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    log_file_format: terrars::PrimField<std::string::String>,
}

impl WorkspaceswebSessionLogger_LogConfiguration_S3 {
    #[doc = "Set the field `bucket_owner`.\n"]
    pub fn set_bucket_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebSessionLogger_LogConfiguration_S3 {
    type O = terrars::BlockAssignable<WorkspaceswebSessionLogger_LogConfiguration_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebSessionLogger_LogConfiguration_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub folder_structure: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_file_format: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebSessionLogger_LogConfiguration_S3 {
    pub fn build(self) -> WorkspaceswebSessionLogger_LogConfiguration_S3 {
        WorkspaceswebSessionLogger_LogConfiguration_S3 {
            bucket: self.bucket,
            bucket_owner: core::default::Default::default(),
            folder_structure: self.folder_structure,
            key_prefix: core::default::Default::default(),
            log_file_format: self.log_file_format,
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebSessionLogger_LogConfiguration_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebSessionLogger_LogConfiguration_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebSessionLogger_LogConfiguration_S3Ref {
        WorkspaceswebSessionLogger_LogConfiguration_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebSessionLogger_LogConfiguration_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `folder_structure` after provisioning.\n"]
    pub fn folder_structure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder_structure", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `log_file_format` after provisioning.\n"]
    pub fn log_file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_file_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebSessionLogger_LogConfigurationDynamic {
    s3: std::option::Option<terrars::DynamicBlock<WorkspaceswebSessionLogger_LogConfiguration_S3>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebSessionLogger_LogConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<WorkspaceswebSessionLogger_LogConfiguration_S3>>,
    dynamic: WorkspaceswebSessionLogger_LogConfigurationDynamic,
}

impl WorkspaceswebSessionLogger_LogConfiguration {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspaceswebSessionLogger_LogConfiguration_S3>>,
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

impl terrars::ToListMappable for WorkspaceswebSessionLogger_LogConfiguration {
    type O = terrars::BlockAssignable<WorkspaceswebSessionLogger_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebSessionLogger_LogConfiguration {}

impl BuildWorkspaceswebSessionLogger_LogConfiguration {
    pub fn build(self) -> WorkspaceswebSessionLogger_LogConfiguration {
        WorkspaceswebSessionLogger_LogConfiguration {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebSessionLogger_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebSessionLogger_LogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebSessionLogger_LogConfigurationRef {
        WorkspaceswebSessionLogger_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebSessionLogger_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<WorkspaceswebSessionLogger_LogConfiguration_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
