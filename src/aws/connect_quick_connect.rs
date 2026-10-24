use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectQuickConnectData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quick_connect_config: std::option::Option<std::vec::Vec<ConnectQuickConnectQuickConnectConfigEl>>,
    dynamic: ConnectQuickConnectDynamic,
}

struct ConnectQuickConnect_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectQuickConnectData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigEl>>,
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> terrars::ListRef<ConnectQuickConnectQuickConnectConfigElRef> {
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
            data: std::cell::RefCell::new(ConnectQuickConnectData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> terrars::ListRef<ConnectQuickConnectQuickConnectConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl ConnectQuickConnectQuickConnectConfigElPhoneConfigEl { }

impl terrars::ToListMappable for ConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    type O = terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    #[doc = ""]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
        ConnectQuickConnectQuickConnectConfigElPhoneConfigEl { phone_number: self.phone_number }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
        ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    contact_flow_id: terrars::PrimField<std::string::String>,
    queue_id: terrars::PrimField<std::string::String>,
}

impl ConnectQuickConnectQuickConnectConfigElQueueConfigEl { }

impl terrars::ToListMappable for ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    type O = terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    #[doc = ""]
    pub contact_flow_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub queue_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
        ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
            contact_flow_id: self.contact_flow_id,
            queue_id: self.queue_id,
        }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
        ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
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
pub struct ConnectQuickConnectQuickConnectConfigElUserConfigEl {
    contact_flow_id: terrars::PrimField<std::string::String>,
    user_id: terrars::PrimField<std::string::String>,
}

impl ConnectQuickConnectQuickConnectConfigElUserConfigEl { }

impl terrars::ToListMappable for ConnectQuickConnectQuickConnectConfigElUserConfigEl {
    type O = terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigElUserConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigElUserConfigEl {
    #[doc = ""]
    pub contact_flow_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnectQuickConnectConfigElUserConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigElUserConfigEl {
        ConnectQuickConnectQuickConnectConfigElUserConfigEl {
            contact_flow_id: self.contact_flow_id,
            user_id: self.user_id,
        }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
        ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
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
struct ConnectQuickConnectQuickConnectConfigElDynamic {
    phone_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    queue_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    user_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigEl {
    quick_connect_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_config: std::option::Option<std::vec::Vec<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_config: std::option::Option<std::vec::Vec<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_config: std::option::Option<std::vec::Vec<ConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
    dynamic: ConnectQuickConnectQuickConnectConfigElDynamic,
}

impl ConnectQuickConnectQuickConnectConfigEl {
    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
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

impl terrars::ToListMappable for ConnectQuickConnectQuickConnectConfigEl {
    type O = terrars::BlockAssignable<ConnectQuickConnectQuickConnectConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigEl {
    #[doc = ""]
    pub quick_connect_type: terrars::PrimField<std::string::String>,
}

impl BuildConnectQuickConnectQuickConnectConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigEl {
        ConnectQuickConnectQuickConnectConfigEl {
            quick_connect_type: self.quick_connect_type,
            phone_config: core::default::Default::default(),
            queue_config: core::default::Default::default(),
            user_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectQuickConnectQuickConnectConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectQuickConnectQuickConnectConfigElRef {
        ConnectQuickConnectQuickConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `quick_connect_type` after provisioning.\n"]
    pub fn quick_connect_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_type", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> terrars::ListRef<ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_config", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_config` after provisioning.\n"]
    pub fn queue_config(&self) -> terrars::ListRef<ConnectQuickConnectQuickConnectConfigElQueueConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue_config", self.base))
    }

    #[doc = "Get a reference to the value of field `user_config` after provisioning.\n"]
    pub fn user_config(&self) -> terrars::ListRef<ConnectQuickConnectQuickConnectConfigElUserConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectQuickConnectDynamic {
    quick_connect_config: std::option::Option<terrars::DynamicBlock<ConnectQuickConnectQuickConnectConfigEl>>,
}
