use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallLoggingConfigurationData {
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
    firewall_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>>,
    dynamic: NetworkfirewallLoggingConfigurationDynamic,
}

struct NetworkfirewallLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallLoggingConfigurationData>,
}

#[derive(Clone)]
pub struct NetworkfirewallLoggingConfiguration(std::rc::Rc<NetworkfirewallLoggingConfiguration_>);

impl NetworkfirewallLoggingConfiguration {
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

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallLoggingConfigurationLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkfirewallLoggingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkfirewallLoggingConfiguration { }

impl terrars::ToListMappable for NetworkfirewallLoggingConfiguration {
    type O = terrars::ListRef<NetworkfirewallLoggingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkfirewallLoggingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkfirewall_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallLoggingConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firewall_arn: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallLoggingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallLoggingConfiguration {
        let out = NetworkfirewallLoggingConfiguration(std::rc::Rc::new(NetworkfirewallLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkfirewallLoggingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                firewall_arn: self.firewall_arn,
                id: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallLoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallLoggingConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallLoggingConfigurationLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    log_destination: terrars::RecField<terrars::PrimField<std::string::String>>,
    log_destination_type: terrars::PrimField<std::string::String>,
    log_type: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl { }

impl terrars::ToListMappable for NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    type O =
        terrars::BlockAssignable<NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    #[doc = ""]
    pub log_destination: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub log_destination_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    pub fn build(self) -> NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
        NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
            log_destination: self.log_destination,
            log_destination_type: self.log_destination_type,
            log_type: self.log_type,
        }
    }
}

pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
        NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallLoggingConfigurationLoggingConfigurationElDynamic {
    log_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination_config: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl>>,
    dynamic: NetworkfirewallLoggingConfigurationLoggingConfigurationElDynamic,
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    #[doc = "Set the field `log_destination_config`.\n"]
    pub fn set_log_destination_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    type O = terrars::BlockAssignable<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallLoggingConfigurationLoggingConfigurationEl {}

impl BuildNetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    pub fn build(self) -> NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
        NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
            log_destination_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
        NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallLoggingConfigurationDynamic {
    logging_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>>,
}
