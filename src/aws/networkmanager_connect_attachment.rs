use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerConnectAttachmentData {
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
    core_network_id: terrars::PrimField<std::string::String>,
    edge_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    transport_attachment_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<std::vec::Vec<NetworkmanagerConnectAttachmentOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkmanagerConnectAttachmentTimeoutsEl>,
    dynamic: NetworkmanagerConnectAttachmentDynamic,
}

struct NetworkmanagerConnectAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkmanagerConnectAttachmentData>,
}

#[derive(Clone)]
pub struct NetworkmanagerConnectAttachment(std::rc::Rc<NetworkmanagerConnectAttachment_>);

impl NetworkmanagerConnectAttachment {
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

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkmanagerConnectAttachmentOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkmanagerConnectAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_policy_rule_number` after provisioning.\n"]
    pub fn attachment_policy_rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.attachment_policy_rule_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_name` after provisioning.\n"]
    pub fn segment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transport_attachment_id` after provisioning.\n"]
    pub fn transport_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<NetworkmanagerConnectAttachmentOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerConnectAttachmentTimeoutsElRef {
        NetworkmanagerConnectAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkmanagerConnectAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkmanagerConnectAttachment { }

impl terrars::ToListMappable for NetworkmanagerConnectAttachment {
    type O = terrars::ListRef<NetworkmanagerConnectAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkmanagerConnectAttachment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkmanager_connect_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerConnectAttachment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub core_network_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub edge_location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub transport_attachment_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkmanagerConnectAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkmanagerConnectAttachment {
        let out = NetworkmanagerConnectAttachment(std::rc::Rc::new(NetworkmanagerConnectAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkmanagerConnectAttachmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                core_network_id: self.core_network_id,
                edge_location: self.edge_location,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transport_attachment_id: self.transport_attachment_id,
                options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerConnectAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkmanagerConnectAttachmentRef {
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

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_policy_rule_number` after provisioning.\n"]
    pub fn attachment_policy_rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.attachment_policy_rule_number", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_name` after provisioning.\n"]
    pub fn segment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transport_attachment_id` after provisioning.\n"]
    pub fn transport_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<NetworkmanagerConnectAttachmentOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerConnectAttachmentTimeoutsElRef {
        NetworkmanagerConnectAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectAttachmentOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkmanagerConnectAttachmentOptionsEl {
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkmanagerConnectAttachmentOptionsEl {
    type O = terrars::BlockAssignable<NetworkmanagerConnectAttachmentOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectAttachmentOptionsEl {}

impl BuildNetworkmanagerConnectAttachmentOptionsEl {
    pub fn build(self) -> NetworkmanagerConnectAttachmentOptionsEl {
        NetworkmanagerConnectAttachmentOptionsEl { protocol: core::default::Default::default() }
    }
}

pub struct NetworkmanagerConnectAttachmentOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectAttachmentOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkmanagerConnectAttachmentOptionsElRef {
        NetworkmanagerConnectAttachmentOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectAttachmentOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkmanagerConnectAttachmentTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkmanagerConnectAttachmentTimeoutsEl {
    type O = terrars::BlockAssignable<NetworkmanagerConnectAttachmentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectAttachmentTimeoutsEl {}

impl BuildNetworkmanagerConnectAttachmentTimeoutsEl {
    pub fn build(self) -> NetworkmanagerConnectAttachmentTimeoutsEl {
        NetworkmanagerConnectAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerConnectAttachmentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectAttachmentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkmanagerConnectAttachmentTimeoutsElRef {
        NetworkmanagerConnectAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectAttachmentTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkmanagerConnectAttachmentDynamic {
    options: std::option::Option<terrars::DynamicBlock<NetworkmanagerConnectAttachmentOptionsEl>>,
}
