#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectQuickConnectData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quick_connect_config: std::option::Option<std::vec::Vec<ConnectQuickConnect_QuickConnectConfig>>,
    dynamic: ConnectQuickConnectDynamic,
}

struct ConnectQuickConnect_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectQuickConnectData_>,
}

#[derive(Clone)]
pub struct ConnectQuickConnect(std::rc::Rc<ConnectQuickConnect_>);

impl ConnectQuickConnect {
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

    #[doc = "Set the field `quick_connect_config`.\n"]
    pub fn set_quick_connect_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quick_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quick_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> terrars::ListRef<ConnectQuickConnect_QuickConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectQuickConnect {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectQuickConnect { }

impl terrars::ToListMappable for ConnectQuickConnect {
    type O = terrars::ListRef<ConnectQuickConnectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectQuickConnect_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_quick_connect".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectQuickConnect {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnect {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectQuickConnect {
        let out = ConnectQuickConnect(std::rc::Rc::new(ConnectQuickConnect_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectQuickConnectData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                quick_connect_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectQuickConnectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectQuickConnectRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> terrars::ListRef<ConnectQuickConnect_QuickConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ConnectQuickConnectDynamic {
    quick_connect_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnect_QuickConnectConfig>>,
}

#[derive(Serialize)]
pub struct ConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    phone_number: terrars::PrimField<std::string::String>,
}

impl ConnectQuickConnect_QuickConnectConfig_PhoneConfig { }

impl terrars::ToListMappable for ConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    type O = terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig_PhoneConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    #[doc = ""]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    pub fn build(self) -> ConnectQuickConnect_QuickConnectConfig_PhoneConfig {
        ConnectQuickConnect_QuickConnectConfig_PhoneConfig { phone_number: self.phone_number }
    }
}

#[allow(dead_code)]
pub struct ConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
        ConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnect_QuickConnectConfig_QueueConfig {
    contact_flow_id: terrars::PrimField<std::string::String>,
    queue_id: terrars::PrimField<std::string::String>,
}

impl ConnectQuickConnect_QuickConnectConfig_QueueConfig { }

impl terrars::ToListMappable for ConnectQuickConnect_QuickConnectConfig_QueueConfig {
    type O = terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig_QueueConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnect_QuickConnectConfig_QueueConfig {
    #[doc = ""]
    pub contact_flow_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub queue_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnect_QuickConnectConfig_QueueConfig {
    pub fn build(self) -> ConnectQuickConnect_QuickConnectConfig_QueueConfig {
        ConnectQuickConnect_QuickConnectConfig_QueueConfig {
            contact_flow_id: self.contact_flow_id,
            queue_id: self.queue_id,
        }
    }
}

#[allow(dead_code)]
pub struct ConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
        ConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnect_QuickConnectConfig_UserConfig {
    contact_flow_id: terrars::PrimField<std::string::String>,
    user_id: terrars::PrimField<std::string::String>,
}

impl ConnectQuickConnect_QuickConnectConfig_UserConfig { }

impl terrars::ToListMappable for ConnectQuickConnect_QuickConnectConfig_UserConfig {
    type O = terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig_UserConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnect_QuickConnectConfig_UserConfig {
    #[doc = ""]
    pub contact_flow_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnect_QuickConnectConfig_UserConfig {
    pub fn build(self) -> ConnectQuickConnect_QuickConnectConfig_UserConfig {
        ConnectQuickConnect_QuickConnectConfig_UserConfig {
            contact_flow_id: self.contact_flow_id,
            user_id: self.user_id,
        }
    }
}

#[allow(dead_code)]
pub struct ConnectQuickConnect_QuickConnectConfig_UserConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnect_QuickConnectConfig_UserConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectQuickConnect_QuickConnectConfig_UserConfigRef {
        ConnectQuickConnect_QuickConnectConfig_UserConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnect_QuickConnectConfig_UserConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectQuickConnect_QuickConnectConfigDynamic {
    phone_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnect_QuickConnectConfig_PhoneConfig>>,
    queue_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnect_QuickConnectConfig_QueueConfig>>,
    user_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnect_QuickConnectConfig_UserConfig>>,
}

#[derive(Serialize)]
pub struct ConnectQuickConnect_QuickConnectConfig {
    quick_connect_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_config: std::option::Option<std::vec::Vec<ConnectQuickConnect_QuickConnectConfig_PhoneConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_config: std::option::Option<std::vec::Vec<ConnectQuickConnect_QuickConnectConfig_QueueConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_config: std::option::Option<std::vec::Vec<ConnectQuickConnect_QuickConnectConfig_UserConfig>>,
    dynamic: ConnectQuickConnect_QuickConnectConfigDynamic,
}

impl ConnectQuickConnect_QuickConnectConfig {
    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig_PhoneConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phone_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `queue_config`.\n"]
    pub fn set_queue_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig_QueueConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.queue_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.queue_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_config`.\n"]
    pub fn set_user_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig_UserConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConnectQuickConnect_QuickConnectConfig {
    type O = terrars::BlockAssignable<ConnectQuickConnect_QuickConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnect_QuickConnectConfig {
    #[doc = ""]
    pub quick_connect_type: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnect_QuickConnectConfig {
    pub fn build(self) -> ConnectQuickConnect_QuickConnectConfig {
        ConnectQuickConnect_QuickConnectConfig {
            quick_connect_type: self.quick_connect_type,
            phone_config: core::default::Default::default(),
            queue_config: core::default::Default::default(),
            user_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectQuickConnect_QuickConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnect_QuickConnectConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectQuickConnect_QuickConnectConfigRef {
        ConnectQuickConnect_QuickConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnect_QuickConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `quick_connect_type` after provisioning.\n"]
    pub fn quick_connect_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_type", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> terrars::ListRef<ConnectQuickConnect_QuickConnectConfig_PhoneConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_config", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_config` after provisioning.\n"]
    pub fn queue_config(&self) -> terrars::ListRef<ConnectQuickConnect_QuickConnectConfig_QueueConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue_config", self.base))
    }

    #[doc = "Get a reference to the value of field `user_config` after provisioning.\n"]
    pub fn user_config(&self) -> terrars::ListRef<ConnectQuickConnect_QuickConnectConfig_UserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_config", self.base))
    }
}
