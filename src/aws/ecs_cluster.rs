use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsClusterData {
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
    capacity_providers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<EcsClusterConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_capacity_provider_strategy: std::option::Option<std::vec::Vec<EcsClusterDefaultCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_connect_defaults: std::option::Option<std::vec::Vec<EcsClusterServiceConnectDefaultsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    setting: std::option::Option<std::vec::Vec<EcsClusterSettingEl>>,
    dynamic: EcsClusterDynamic,
}

struct EcsCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsClusterData>,
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

    #[doc = "Set the field `capacity_providers`.\n"]
    pub fn set_capacity_providers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().capacity_providers = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsClusterConfigurationEl>>,
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

    #[doc = "Set the field `default_capacity_provider_strategy`.\n"]
    pub fn set_default_capacity_provider_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsClusterDefaultCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_capacity_provider_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_capacity_provider_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_connect_defaults`.\n"]
    pub fn set_service_connect_defaults(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsClusterServiceConnectDefaultsEl>>,
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
    pub fn set_setting(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsClusterSettingEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `capacity_providers` after provisioning.\n"]
    pub fn capacity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
    pub fn configuration(&self) -> terrars::ListRef<EcsClusterConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_defaults` after provisioning.\n"]
    pub fn service_connect_defaults(&self) -> terrars::ListRef<EcsClusterServiceConnectDefaultsElRef> {
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
            data: std::cell::RefCell::new(EcsClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_providers: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                default_capacity_provider_strategy: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `capacity_providers` after provisioning.\n"]
    pub fn capacity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
    pub fn configuration(&self) -> terrars::ListRef<EcsClusterConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_connect_defaults` after provisioning.\n"]
    pub fn service_connect_defaults(&self) -> terrars::ListRef<EcsClusterServiceConnectDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_connect_defaults", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
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

impl EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
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

impl terrars::ToListMappable for EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
    type O = terrars::BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {}

impl BuildEcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
        EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
            cloud_watch_encryption_enabled: core::default::Default::default(),
            cloud_watch_log_group_name: core::default::Default::default(),
            s3_bucket_encryption_enabled: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
        EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
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
struct EcsClusterConfigurationElExecuteCommandConfigurationElDynamic {
    log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>>,
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationElExecuteCommandConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>>,
    dynamic: EcsClusterConfigurationElExecuteCommandConfigurationElDynamic,
}

impl EcsClusterConfigurationElExecuteCommandConfigurationEl {
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
                        ::BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>,
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

impl terrars::ToListMappable for EcsClusterConfigurationElExecuteCommandConfigurationEl {
    type O = terrars::BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationElExecuteCommandConfigurationEl {}

impl BuildEcsClusterConfigurationElExecuteCommandConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationElExecuteCommandConfigurationEl {
        EcsClusterConfigurationElExecuteCommandConfigurationEl {
            kms_key_id: core::default::Default::default(),
            logging: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElExecuteCommandConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterConfigurationElExecuteCommandConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsClusterConfigurationElExecuteCommandConfigurationElRef {
        EcsClusterConfigurationElExecuteCommandConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElExecuteCommandConfigurationElRef {
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
    ) -> terrars::ListRef<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsClusterConfigurationElDynamic {
    execute_command_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsClusterConfigurationElExecuteCommandConfigurationEl>>,
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execute_command_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsClusterConfigurationElExecuteCommandConfigurationEl>>,
    dynamic: EcsClusterConfigurationElDynamic,
}

impl EcsClusterConfigurationEl {
    #[doc = "Set the field `execute_command_configuration`.\n"]
    pub fn set_execute_command_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationEl>>,
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
}

impl terrars::ToListMappable for EcsClusterConfigurationEl {
    type O = terrars::BlockAssignable<EcsClusterConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationEl {}

impl BuildEcsClusterConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationEl {
        EcsClusterConfigurationEl {
            execute_command_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsClusterConfigurationElRef {
        EcsClusterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execute_command_configuration` after provisioning.\n"]
    pub fn execute_command_configuration(
        &self,
    ) -> terrars::ListRef<EcsClusterConfigurationElExecuteCommandConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execute_command_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsClusterDefaultCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsClusterDefaultCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsClusterDefaultCapacityProviderStrategyEl {
    type O = terrars::BlockAssignable<EcsClusterDefaultCapacityProviderStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterDefaultCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildEcsClusterDefaultCapacityProviderStrategyEl {
    pub fn build(self) -> EcsClusterDefaultCapacityProviderStrategyEl {
        EcsClusterDefaultCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterDefaultCapacityProviderStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterDefaultCapacityProviderStrategyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsClusterDefaultCapacityProviderStrategyElRef {
        EcsClusterDefaultCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterDefaultCapacityProviderStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsClusterServiceConnectDefaultsEl {
    namespace: terrars::PrimField<std::string::String>,
}

impl EcsClusterServiceConnectDefaultsEl { }

impl terrars::ToListMappable for EcsClusterServiceConnectDefaultsEl {
    type O = terrars::BlockAssignable<EcsClusterServiceConnectDefaultsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterServiceConnectDefaultsEl {
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildEcsClusterServiceConnectDefaultsEl {
    pub fn build(self) -> EcsClusterServiceConnectDefaultsEl {
        EcsClusterServiceConnectDefaultsEl { namespace: self.namespace }
    }
}

pub struct EcsClusterServiceConnectDefaultsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterServiceConnectDefaultsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsClusterServiceConnectDefaultsElRef {
        EcsClusterServiceConnectDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterServiceConnectDefaultsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsClusterSettingEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl EcsClusterSettingEl { }

impl terrars::ToListMappable for EcsClusterSettingEl {
    type O = terrars::BlockAssignable<EcsClusterSettingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterSettingEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildEcsClusterSettingEl {
    pub fn build(self) -> EcsClusterSettingEl {
        EcsClusterSettingEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct EcsClusterSettingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterSettingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsClusterSettingElRef {
        EcsClusterSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterSettingElRef {
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

#[derive(Serialize, Default)]
struct EcsClusterDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<EcsClusterConfigurationEl>>,
    default_capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<EcsClusterDefaultCapacityProviderStrategyEl>>,
    service_connect_defaults: std::option::Option<terrars::DynamicBlock<EcsClusterServiceConnectDefaultsEl>>,
    setting: std::option::Option<terrars::DynamicBlock<EcsClusterSettingEl>>,
}
