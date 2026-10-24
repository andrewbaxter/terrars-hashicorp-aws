use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApprunnerServiceData {
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
    auto_scaling_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    service_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceHealthCheckConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceInstanceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observability_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceObservabilityConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationEl>>,
    dynamic: ApprunnerServiceDynamic,
}

struct ApprunnerService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApprunnerServiceData>,
}

#[derive(Clone)]
pub struct ApprunnerService(std::rc::Rc<ApprunnerService_>);

impl ApprunnerService {
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

    #[doc = "Set the field `auto_scaling_configuration_arn`.\n"]
    pub fn set_auto_scaling_configuration_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().auto_scaling_configuration_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check_configuration`.\n"]
    pub fn set_health_check_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceHealthCheckConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_configuration`.\n"]
    pub fn set_instance_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceInstanceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `observability_configuration`.\n"]
    pub fn set_observability_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceObservabilityConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().observability_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.observability_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_configuration_arn` after provisioning.\n"]
    pub fn auto_scaling_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_scaling_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_url` after provisioning.\n"]
    pub fn service_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<ApprunnerServiceEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_configuration` after provisioning.\n"]
    pub fn health_check_configuration(&self) -> terrars::ListRef<ApprunnerServiceHealthCheckConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> terrars::ListRef<ApprunnerServiceInstanceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<ApprunnerServiceNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `observability_configuration` after provisioning.\n"]
    pub fn observability_configuration(&self) -> terrars::ListRef<ApprunnerServiceObservabilityConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.observability_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(&self) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for ApprunnerService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApprunnerService { }

impl terrars::ToListMappable for ApprunnerService {
    type O = terrars::ListRef<ApprunnerServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApprunnerService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_apprunner_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApprunnerService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerService {
    pub fn build(self, stack: &mut terrars::Stack) -> ApprunnerService {
        let out = ApprunnerService(std::rc::Rc::new(ApprunnerService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApprunnerServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_scaling_configuration_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                service_name: self.service_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                health_check_configuration: core::default::Default::default(),
                instance_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                observability_configuration: core::default::Default::default(),
                source_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApprunnerServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApprunnerServiceRef {
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

    #[doc = "Get a reference to the value of field `auto_scaling_configuration_arn` after provisioning.\n"]
    pub fn auto_scaling_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_scaling_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_url` after provisioning.\n"]
    pub fn service_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<ApprunnerServiceEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_configuration` after provisioning.\n"]
    pub fn health_check_configuration(&self) -> terrars::ListRef<ApprunnerServiceHealthCheckConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> terrars::ListRef<ApprunnerServiceInstanceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<ApprunnerServiceNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `observability_configuration` after provisioning.\n"]
    pub fn observability_configuration(&self) -> terrars::ListRef<ApprunnerServiceObservabilityConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.observability_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(&self) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceEncryptionConfigurationEl {
    kms_key: terrars::PrimField<std::string::String>,
}

impl ApprunnerServiceEncryptionConfigurationEl { }

impl terrars::ToListMappable for ApprunnerServiceEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceEncryptionConfigurationEl {
    #[doc = ""]
    pub kms_key: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerServiceEncryptionConfigurationEl {
    pub fn build(self) -> ApprunnerServiceEncryptionConfigurationEl {
        ApprunnerServiceEncryptionConfigurationEl { kms_key: self.kms_key }
    }
}

pub struct ApprunnerServiceEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceEncryptionConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApprunnerServiceEncryptionConfigurationElRef {
        ApprunnerServiceEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceHealthCheckConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl ApprunnerServiceHealthCheckConfigurationEl {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceHealthCheckConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceHealthCheckConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceHealthCheckConfigurationEl {}

impl BuildApprunnerServiceHealthCheckConfigurationEl {
    pub fn build(self) -> ApprunnerServiceHealthCheckConfigurationEl {
        ApprunnerServiceHealthCheckConfigurationEl {
            healthy_threshold: core::default::Default::default(),
            interval: core::default::Default::default(),
            path: core::default::Default::default(),
            protocol: core::default::Default::default(),
            timeout: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceHealthCheckConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceHealthCheckConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApprunnerServiceHealthCheckConfigurationElRef {
        ApprunnerServiceHealthCheckConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceHealthCheckConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceInstanceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApprunnerServiceInstanceConfigurationEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_role_arn`.\n"]
    pub fn set_instance_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceInstanceConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceInstanceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceInstanceConfigurationEl {}

impl BuildApprunnerServiceInstanceConfigurationEl {
    pub fn build(self) -> ApprunnerServiceInstanceConfigurationEl {
        ApprunnerServiceInstanceConfigurationEl {
            cpu: core::default::Default::default(),
            instance_role_arn: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceInstanceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceInstanceConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApprunnerServiceInstanceConfigurationElRef {
        ApprunnerServiceInstanceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceInstanceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_role_arn` after provisioning.\n"]
    pub fn instance_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    #[doc = "Set the field `egress_type`.\n"]
    pub fn set_egress_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.egress_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_connector_arn`.\n"]
    pub fn set_vpc_connector_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_connector_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceNetworkConfigurationElEgressConfigurationEl {}

impl BuildApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    pub fn build(self) -> ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
        ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
            egress_type: core::default::Default::default(),
            vpc_connector_arn: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
        ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_type` after provisioning.\n"]
    pub fn egress_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_connector_arn` after provisioning.\n"]
    pub fn vpc_connector_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
}

impl ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    #[doc = "Set the field `is_publicly_accessible`.\n"]
    pub fn set_is_publicly_accessible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_publicly_accessible = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceNetworkConfigurationElIngressConfigurationEl {}

impl BuildApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    pub fn build(self) -> ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
        ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
            is_publicly_accessible: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
        ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_publicly_accessible` after provisioning.\n"]
    pub fn is_publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_publicly_accessible", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApprunnerServiceNetworkConfigurationElDynamic {
    egress_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>>,
    ingress_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>>,
}

#[derive(Serialize)]
pub struct ApprunnerServiceNetworkConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_configuration: std::option::Option<std::vec::Vec<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_configuration: std
    ::option
    ::Option<std::vec::Vec<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>>,
    dynamic: ApprunnerServiceNetworkConfigurationElDynamic,
}

impl ApprunnerServiceNetworkConfigurationEl {
    #[doc = "Set the field `egress_configuration`.\n"]
    pub fn set_egress_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_configuration`.\n"]
    pub fn set_ingress_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceNetworkConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceNetworkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceNetworkConfigurationEl {}

impl BuildApprunnerServiceNetworkConfigurationEl {
    pub fn build(self) -> ApprunnerServiceNetworkConfigurationEl {
        ApprunnerServiceNetworkConfigurationEl {
            egress_configuration: core::default::Default::default(),
            ingress_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceNetworkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceNetworkConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApprunnerServiceNetworkConfigurationElRef {
        ApprunnerServiceNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceNetworkConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_configuration` after provisioning.\n"]
    pub fn egress_configuration(
        &self,
    ) -> terrars::ListRef<ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_configuration` after provisioning.\n"]
    pub fn ingress_configuration(
        &self,
    ) -> terrars::ListRef<ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceObservabilityConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observability_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    observability_enabled: terrars::PrimField<bool>,
}

impl ApprunnerServiceObservabilityConfigurationEl {
    #[doc = "Set the field `observability_configuration_arn`.\n"]
    pub fn set_observability_configuration_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.observability_configuration_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceObservabilityConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceObservabilityConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceObservabilityConfigurationEl {
    #[doc = ""]
    pub observability_enabled: terrars::PrimField<bool>,
}

impl BuildApprunnerServiceObservabilityConfigurationEl {
    pub fn build(self) -> ApprunnerServiceObservabilityConfigurationEl {
        ApprunnerServiceObservabilityConfigurationEl {
            observability_configuration_arn: core::default::Default::default(),
            observability_enabled: self.observability_enabled,
        }
    }
}

pub struct ApprunnerServiceObservabilityConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceObservabilityConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceObservabilityConfigurationElRef {
        ApprunnerServiceObservabilityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceObservabilityConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `observability_configuration_arn` after provisioning.\n"]
    pub fn observability_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observability_configuration_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `observability_enabled` after provisioning.\n"]
    pub fn observability_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observability_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    #[doc = "Set the field `access_role_arn`.\n"]
    pub fn set_access_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_arn`.\n"]
    pub fn set_connection_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {}

impl BuildApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
        ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
            access_role_arn: core::default::Default::default(),
            connection_arn: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
        ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_role_arn` after provisioning.\n"]
    pub fn access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<std::string::String>>,
    runtime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_environment_secrets: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_command: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
    #[doc = "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_environment_secrets`.\n"]
    pub fn set_runtime_environment_secrets(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.runtime_environment_secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_environment_variables`.\n"]
    pub fn set_runtime_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.runtime_environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_command`.\n"]
    pub fn set_start_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_command = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
    type O =
        terrars
        ::BlockAssignable<
            ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
    #[doc = ""]
    pub runtime: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
    pub fn build(
        self,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl {
            build_command: core::default::Default::default(),
            port: core::default::Default::default(),
            runtime: self.runtime,
            runtime_environment_secrets: core::default::Default::default(),
            runtime_environment_variables: core::default::Default::default(),
            start_command: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_environment_secrets` after provisioning.\n"]
    pub fn runtime_environment_secrets(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.runtime_environment_secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_environment_variables` after provisioning.\n"]
    pub fn runtime_environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.runtime_environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `start_command` after provisioning.\n"]
    pub fn start_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_command", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElDynamic {
    code_configuration_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    configuration_source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_configuration_values: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl>,
    >,
    dynamic: ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElDynamic,
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    #[doc = "Set the field `code_configuration_values`.\n"]
    pub fn set_code_configuration_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_configuration_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_configuration_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    #[doc = ""]
    pub configuration_source: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
            configuration_source: self.configuration_source,
            code_configuration_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_source` after provisioning.\n"]
    pub fn configuration_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_source", self.base))
    }

    #[doc = "Get a reference to the value of field `code_configuration_values` after provisioning.\n"]
    pub fn code_configuration_values(
        &self,
    ) -> terrars
    ::ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_configuration_values", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl { }

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElCodeRepositoryElDynamic {
    code_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>>,
    source_code_version: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>>,
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    repository_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_configuration: std
    ::option
    ::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_code_version: std
    ::option
    ::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>>,
    dynamic: ApprunnerServiceSourceConfigurationElCodeRepositoryElDynamic,
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    #[doc = "Set the field `code_configuration`.\n"]
    pub fn set_code_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_code_version`.\n"]
    pub fn set_source_code_version(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_code_version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_code_version = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
            repository_url: self.repository_url,
            code_configuration: core::default::Default::default(),
            source_code_version: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }

    #[doc = "Get a reference to the value of field `code_configuration` after provisioning.\n"]
    pub fn code_configuration(
        &self,
    ) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_code_version` after provisioning.\n"]
    pub fn source_code_version(
        &self,
    ) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_code_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_environment_secrets: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_command: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_environment_secrets`.\n"]
    pub fn set_runtime_environment_secrets(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.runtime_environment_secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_environment_variables`.\n"]
    pub fn set_runtime_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.runtime_environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_command`.\n"]
    pub fn set_start_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_command = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {}

impl BuildApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
        ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
            port: core::default::Default::default(),
            runtime_environment_secrets: core::default::Default::default(),
            runtime_environment_variables: core::default::Default::default(),
            start_command: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
        ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_environment_secrets` after provisioning.\n"]
    pub fn runtime_environment_secrets(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.runtime_environment_secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_environment_variables` after provisioning.\n"]
    pub fn runtime_environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.runtime_environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `start_command` after provisioning.\n"]
    pub fn start_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_command", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElImageRepositoryElDynamic {
    image_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>>,
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElImageRepositoryEl {
    image_identifier: terrars::PrimField<std::string::String>,
    image_repository_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_configuration: std
    ::option
    ::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>>,
    dynamic: ApprunnerServiceSourceConfigurationElImageRepositoryElDynamic,
}

impl ApprunnerServiceSourceConfigurationElImageRepositoryEl {
    #[doc = "Set the field `image_configuration`.\n"]
    pub fn set_image_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationElImageRepositoryEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationElImageRepositoryEl {
    #[doc = ""]
    pub image_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_repository_type: terrars::PrimField<std::string::String>,
}

impl BuildApprunnerServiceSourceConfigurationElImageRepositoryEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElImageRepositoryEl {
        ApprunnerServiceSourceConfigurationElImageRepositoryEl {
            image_identifier: self.image_identifier,
            image_repository_type: self.image_repository_type,
            image_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
        ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_identifier` after provisioning.\n"]
    pub fn image_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `image_repository_type` after provisioning.\n"]
    pub fn image_repository_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_repository_type", self.base))
    }

    #[doc = "Get a reference to the value of field `image_configuration` after provisioning.\n"]
    pub fn image_configuration(
        &self,
    ) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElDynamic {
    authentication_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>>,
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>>,
    image_repository: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationElImageRepositoryEl>>,
}

#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_deployments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_configuration: std
    ::option
    ::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std::option::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_repository: std::option::Option<std::vec::Vec<ApprunnerServiceSourceConfigurationElImageRepositoryEl>>,
    dynamic: ApprunnerServiceSourceConfigurationElDynamic,
}

impl ApprunnerServiceSourceConfigurationEl {
    #[doc = "Set the field `auto_deployments_enabled`.\n"]
    pub fn set_auto_deployments_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_deployments_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_repository`.\n"]
    pub fn set_image_repository(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApprunnerServiceSourceConfigurationEl {
    type O = terrars::BlockAssignable<ApprunnerServiceSourceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApprunnerServiceSourceConfigurationEl {}

impl BuildApprunnerServiceSourceConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationEl {
        ApprunnerServiceSourceConfigurationEl {
            auto_deployments_enabled: core::default::Default::default(),
            authentication_configuration: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            image_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApprunnerServiceSourceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApprunnerServiceSourceConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApprunnerServiceSourceConfigurationElRef {
        ApprunnerServiceSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApprunnerServiceSourceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_deployments_enabled` after provisioning.\n"]
    pub fn auto_deployments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_deployments_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(
        &self,
    ) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `code_repository` after provisioning.\n"]
    pub fn code_repository(&self) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `image_repository` after provisioning.\n"]
    pub fn image_repository(&self) -> terrars::ListRef<ApprunnerServiceSourceConfigurationElImageRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApprunnerServiceDynamic {
    encryption_configuration: std::option::Option<terrars::DynamicBlock<ApprunnerServiceEncryptionConfigurationEl>>,
    health_check_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceHealthCheckConfigurationEl>>,
    instance_configuration: std::option::Option<terrars::DynamicBlock<ApprunnerServiceInstanceConfigurationEl>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<ApprunnerServiceNetworkConfigurationEl>>,
    observability_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ApprunnerServiceObservabilityConfigurationEl>>,
    source_configuration: std::option::Option<terrars::DynamicBlock<ApprunnerServiceSourceConfigurationEl>>,
}
