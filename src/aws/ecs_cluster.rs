#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsClusterData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<EcsCluster_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_connect_defaults: std::option::Option<std::vec::Vec<EcsCluster_ServiceConnectDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    setting: std::option::Option<std::vec::Vec<EcsCluster_Setting>>,
    dynamic: EcsClusterDynamic,
}

struct EcsCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsClusterData_>,
}

#[derive(Clone)]
pub struct EcsCluster(std::rc::Rc<EcsCluster_>);

impl EcsCluster {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCluster_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_connect_defaults`.\n"]
    pub fn set_service_connect_defaults(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCluster_ServiceConnectDefaults>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_connect_defaults = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_connect_defaults = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `setting`.\n"]
    pub fn set_setting(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsCluster_Setting>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.setting = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<EcsCluster_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_defaults` after provisioning.\n"]
    pub fn service_connect_defaults(&self) -> terrars::ListRef<EcsCluster_ServiceConnectDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_defaults", self.extract_ref()))
    }
}

impl terrars::Referable for EcsCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsCluster { }

impl terrars::ToListMappable for EcsCluster {
    type O = terrars::ListRef<EcsClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsCluster {
        let out = EcsCluster(std::rc::Rc::new(EcsCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                service_connect_defaults: core::default::Default::default(),
                setting: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsClusterRef {
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<EcsCluster_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_defaults` after provisioning.\n"]
    pub fn service_connect_defaults(&self) -> terrars::ListRef<EcsCluster_ServiceConnectDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_defaults", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcsClusterDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<EcsCluster_Configuration>>,
    service_connect_defaults: std::option::Option<terrars::DynamicBlock<EcsCluster_ServiceConnectDefaults>>,
    setting: std::option::Option<terrars::DynamicBlock<EcsCluster_Setting>>,
}

#[derive(Serialize)]
pub struct EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_encryption_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_encryption_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {
    #[doc = "Set the field `cloud_watch_encryption_enabled`.\n"]
    pub fn set_cloud_watch_encryption_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloud_watch_encryption_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_log_group_name`.\n"]
    pub fn set_cloud_watch_log_group_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_watch_log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_encryption_enabled`.\n"]
    pub fn set_s3_bucket_encryption_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.s3_bucket_encryption_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {
    type O = terrars::BlockAssignable<EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {}

impl BuildEcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {
    pub fn build(self) -> EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {
        EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration {
            cloud_watch_encryption_enabled: core::default::Default::default(),
            cloud_watch_log_group_name: core::default::Default::default(),
            s3_bucket_encryption_enabled: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfigurationRef {
        EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_encryption_enabled` after provisioning.\n"]
    pub fn cloud_watch_encryption_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_encryption_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_name` after provisioning.\n"]
    pub fn cloud_watch_log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_encryption_enabled` after provisioning.\n"]
    pub fn s3_bucket_encryption_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_encryption_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCluster_Configuration_ExecuteCommandConfigurationDynamic {
    log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration>>,
}

#[derive(Serialize)]
pub struct EcsCluster_Configuration_ExecuteCommandConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration>>,
    dynamic: EcsCluster_Configuration_ExecuteCommandConfigurationDynamic,
}

impl EcsCluster_Configuration_ExecuteCommandConfiguration {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsCluster_Configuration_ExecuteCommandConfiguration {
    type O = terrars::BlockAssignable<EcsCluster_Configuration_ExecuteCommandConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCluster_Configuration_ExecuteCommandConfiguration {}

impl BuildEcsCluster_Configuration_ExecuteCommandConfiguration {
    pub fn build(self) -> EcsCluster_Configuration_ExecuteCommandConfiguration {
        EcsCluster_Configuration_ExecuteCommandConfiguration {
            kms_key_id: core::default::Default::default(),
            logging: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCluster_Configuration_ExecuteCommandConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCluster_Configuration_ExecuteCommandConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCluster_Configuration_ExecuteCommandConfigurationRef {
        EcsCluster_Configuration_ExecuteCommandConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCluster_Configuration_ExecuteCommandConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(
        &self,
    ) -> terrars::ListRef<EcsCluster_Configuration_ExecuteCommandConfiguration_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCluster_Configuration_ManagedStorageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fargate_ephemeral_storage_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsCluster_Configuration_ManagedStorageConfiguration {
    #[doc = "Set the field `fargate_ephemeral_storage_kms_key_id`.\n"]
    pub fn set_fargate_ephemeral_storage_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.fargate_ephemeral_storage_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsCluster_Configuration_ManagedStorageConfiguration {
    type O = terrars::BlockAssignable<EcsCluster_Configuration_ManagedStorageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCluster_Configuration_ManagedStorageConfiguration {}

impl BuildEcsCluster_Configuration_ManagedStorageConfiguration {
    pub fn build(self) -> EcsCluster_Configuration_ManagedStorageConfiguration {
        EcsCluster_Configuration_ManagedStorageConfiguration {
            fargate_ephemeral_storage_kms_key_id: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCluster_Configuration_ManagedStorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCluster_Configuration_ManagedStorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCluster_Configuration_ManagedStorageConfigurationRef {
        EcsCluster_Configuration_ManagedStorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCluster_Configuration_ManagedStorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fargate_ephemeral_storage_kms_key_id` after provisioning.\n"]
    pub fn fargate_ephemeral_storage_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fargate_ephemeral_storage_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCluster_ConfigurationDynamic {
    execute_command_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCluster_Configuration_ExecuteCommandConfiguration>>,
    managed_storage_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCluster_Configuration_ManagedStorageConfiguration>>,
}

#[derive(Serialize)]
pub struct EcsCluster_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execute_command_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsCluster_Configuration_ExecuteCommandConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_storage_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsCluster_Configuration_ManagedStorageConfiguration>>,
    dynamic: EcsCluster_ConfigurationDynamic,
}

impl EcsCluster_Configuration {
    #[doc = "Set the field `execute_command_configuration`.\n"]
    pub fn set_execute_command_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCluster_Configuration_ExecuteCommandConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execute_command_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execute_command_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_storage_configuration`.\n"]
    pub fn set_managed_storage_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCluster_Configuration_ManagedStorageConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsCluster_Configuration {
    type O = terrars::BlockAssignable<EcsCluster_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCluster_Configuration {}

impl BuildEcsCluster_Configuration {
    pub fn build(self) -> EcsCluster_Configuration {
        EcsCluster_Configuration {
            execute_command_configuration: core::default::Default::default(),
            managed_storage_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCluster_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCluster_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsCluster_ConfigurationRef {
        EcsCluster_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCluster_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execute_command_configuration` after provisioning.\n"]
    pub fn execute_command_configuration(
        &self,
    ) -> terrars::ListRef<EcsCluster_Configuration_ExecuteCommandConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execute_command_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_storage_configuration` after provisioning.\n"]
    pub fn managed_storage_configuration(
        &self,
    ) -> terrars::ListRef<EcsCluster_Configuration_ManagedStorageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_storage_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCluster_ServiceConnectDefaults {
    namespace: terrars::PrimField<std::string::String>,
}

impl EcsCluster_ServiceConnectDefaults { }

impl terrars::ToListMappable for EcsCluster_ServiceConnectDefaults {
    type O = terrars::BlockAssignable<EcsCluster_ServiceConnectDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCluster_ServiceConnectDefaults {
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildEcsCluster_ServiceConnectDefaults {
    pub fn build(self) -> EcsCluster_ServiceConnectDefaults {
        EcsCluster_ServiceConnectDefaults { namespace: self.namespace }
    }
}

#[allow(dead_code)]
pub struct EcsCluster_ServiceConnectDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCluster_ServiceConnectDefaultsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsCluster_ServiceConnectDefaultsRef {
        EcsCluster_ServiceConnectDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCluster_ServiceConnectDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCluster_Setting {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl EcsCluster_Setting { }

impl terrars::ToListMappable for EcsCluster_Setting {
    type O = terrars::BlockAssignable<EcsCluster_Setting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCluster_Setting {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildEcsCluster_Setting {
    pub fn build(self) -> EcsCluster_Setting {
        EcsCluster_Setting {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct EcsCluster_SettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCluster_SettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsCluster_SettingRef {
        EcsCluster_SettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCluster_SettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
