use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlobalacceleratorAcceleratorData {
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
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<GlobalacceleratorAcceleratorAttributesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GlobalacceleratorAcceleratorTimeoutsEl>,
    dynamic: GlobalacceleratorAcceleratorDynamic,
}

struct GlobalacceleratorAccelerator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlobalacceleratorAcceleratorData>,
}

#[derive(Clone)]
pub struct GlobalacceleratorAccelerator(std::rc::Rc<GlobalacceleratorAccelerator_>);

impl GlobalacceleratorAccelerator {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ip_addresses = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlobalacceleratorAcceleratorAttributesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GlobalacceleratorAcceleratorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> terrars::ListRef<GlobalacceleratorAcceleratorIpSetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_sets", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<GlobalacceleratorAcceleratorAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorAcceleratorTimeoutsElRef {
        GlobalacceleratorAcceleratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GlobalacceleratorAccelerator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlobalacceleratorAccelerator { }

impl terrars::ToListMappable for GlobalacceleratorAccelerator {
    type O = terrars::ListRef<GlobalacceleratorAcceleratorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlobalacceleratorAccelerator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_globalaccelerator_accelerator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlobalacceleratorAccelerator {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlobalacceleratorAccelerator {
    pub fn build(self, stack: &mut terrars::Stack) -> GlobalacceleratorAccelerator {
        let out = GlobalacceleratorAccelerator(std::rc::Rc::new(GlobalacceleratorAccelerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlobalacceleratorAcceleratorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                ip_addresses: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlobalacceleratorAcceleratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorAcceleratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlobalacceleratorAcceleratorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> terrars::ListRef<GlobalacceleratorAcceleratorIpSetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_sets", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<GlobalacceleratorAcceleratorAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorAcceleratorTimeoutsElRef {
        GlobalacceleratorAcceleratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorAcceleratorIpSetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlobalacceleratorAcceleratorIpSetsEl {
    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlobalacceleratorAcceleratorIpSetsEl {
    type O = terrars::BlockAssignable<GlobalacceleratorAcceleratorIpSetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorAcceleratorIpSetsEl {}

impl BuildGlobalacceleratorAcceleratorIpSetsEl {
    pub fn build(self) -> GlobalacceleratorAcceleratorIpSetsEl {
        GlobalacceleratorAcceleratorIpSetsEl {
            ip_addresses: core::default::Default::default(),
            ip_family: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorAcceleratorIpSetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorAcceleratorIpSetsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlobalacceleratorAcceleratorIpSetsElRef {
        GlobalacceleratorAcceleratorIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorAcceleratorIpSetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorAcceleratorAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_logs_s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_logs_s3_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlobalacceleratorAcceleratorAttributesEl {
    #[doc = "Set the field `flow_logs_enabled`.\n"]
    pub fn set_flow_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flow_logs_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow_logs_s3_bucket`.\n"]
    pub fn set_flow_logs_s3_bucket(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flow_logs_s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow_logs_s3_prefix`.\n"]
    pub fn set_flow_logs_s3_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flow_logs_s3_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlobalacceleratorAcceleratorAttributesEl {
    type O = terrars::BlockAssignable<GlobalacceleratorAcceleratorAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorAcceleratorAttributesEl {}

impl BuildGlobalacceleratorAcceleratorAttributesEl {
    pub fn build(self) -> GlobalacceleratorAcceleratorAttributesEl {
        GlobalacceleratorAcceleratorAttributesEl {
            flow_logs_enabled: core::default::Default::default(),
            flow_logs_s3_bucket: core::default::Default::default(),
            flow_logs_s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorAcceleratorAttributesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorAcceleratorAttributesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlobalacceleratorAcceleratorAttributesElRef {
        GlobalacceleratorAcceleratorAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorAcceleratorAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_logs_enabled` after provisioning.\n"]
    pub fn flow_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `flow_logs_s3_bucket` after provisioning.\n"]
    pub fn flow_logs_s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `flow_logs_s3_prefix` after provisioning.\n"]
    pub fn flow_logs_s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_s3_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorAcceleratorTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlobalacceleratorAcceleratorTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlobalacceleratorAcceleratorTimeoutsEl {
    type O = terrars::BlockAssignable<GlobalacceleratorAcceleratorTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorAcceleratorTimeoutsEl {}

impl BuildGlobalacceleratorAcceleratorTimeoutsEl {
    pub fn build(self) -> GlobalacceleratorAcceleratorTimeoutsEl {
        GlobalacceleratorAcceleratorTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorAcceleratorTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorAcceleratorTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlobalacceleratorAcceleratorTimeoutsElRef {
        GlobalacceleratorAcceleratorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorAcceleratorTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlobalacceleratorAcceleratorDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<GlobalacceleratorAcceleratorAttributesEl>>,
}
