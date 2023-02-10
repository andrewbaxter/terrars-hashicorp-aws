use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlobalacceleratorAcceleratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<GlobalacceleratorAcceleratorAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GlobalacceleratorAcceleratorTimeoutsEl>,
    dynamic: GlobalacceleratorAcceleratorDynamic,
}

struct GlobalacceleratorAccelerator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlobalacceleratorAcceleratorData>,
}

#[derive(Clone)]
pub struct GlobalacceleratorAccelerator(Rc<GlobalacceleratorAccelerator_>);

impl GlobalacceleratorAccelerator {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ip_addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `attributes`.\n"]
    pub fn set_attributes(self, v: impl Into<BlockAssignable<GlobalacceleratorAcceleratorAttributesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GlobalacceleratorAcceleratorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> ListRef<GlobalacceleratorAcceleratorIpSetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_sets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<GlobalacceleratorAcceleratorAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorAcceleratorTimeoutsElRef {
        GlobalacceleratorAcceleratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for GlobalacceleratorAccelerator {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GlobalacceleratorAccelerator { }

impl ToListMappable for GlobalacceleratorAccelerator {
    type O = ListRef<GlobalacceleratorAcceleratorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlobalacceleratorAccelerator_ {
    fn extract_resource_type(&self) -> String {
        "aws_globalaccelerator_accelerator".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlobalacceleratorAccelerator {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildGlobalacceleratorAccelerator {
    pub fn build(self, stack: &mut Stack) -> GlobalacceleratorAccelerator {
        let out = GlobalacceleratorAccelerator(Rc::new(GlobalacceleratorAccelerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlobalacceleratorAcceleratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                ip_addresses: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlobalacceleratorAcceleratorRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorAcceleratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlobalacceleratorAcceleratorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> ListRef<GlobalacceleratorAcceleratorIpSetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_sets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<GlobalacceleratorAcceleratorAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorAcceleratorTimeoutsElRef {
        GlobalacceleratorAcceleratorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorAcceleratorIpSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_family: Option<PrimField<String>>,
}

impl GlobalacceleratorAcceleratorIpSetsEl {
    #[doc= "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_family = Some(v.into());
        self
    }
}

impl ToListMappable for GlobalacceleratorAcceleratorIpSetsEl {
    type O = BlockAssignable<GlobalacceleratorAcceleratorIpSetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorAcceleratorIpSetsElRef {
    fn new(shared: StackShared, base: String) -> GlobalacceleratorAcceleratorIpSetsElRef {
        GlobalacceleratorAcceleratorIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorAcceleratorIpSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorAcceleratorAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_s3_prefix: Option<PrimField<String>>,
}

impl GlobalacceleratorAcceleratorAttributesEl {
    #[doc= "Set the field `flow_logs_enabled`.\n"]
    pub fn set_flow_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.flow_logs_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `flow_logs_s3_bucket`.\n"]
    pub fn set_flow_logs_s3_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow_logs_s3_bucket = Some(v.into());
        self
    }

    #[doc= "Set the field `flow_logs_s3_prefix`.\n"]
    pub fn set_flow_logs_s3_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow_logs_s3_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for GlobalacceleratorAcceleratorAttributesEl {
    type O = BlockAssignable<GlobalacceleratorAcceleratorAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorAcceleratorAttributesElRef {
    fn new(shared: StackShared, base: String) -> GlobalacceleratorAcceleratorAttributesElRef {
        GlobalacceleratorAcceleratorAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorAcceleratorAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `flow_logs_enabled` after provisioning.\n"]
    pub fn flow_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `flow_logs_s3_bucket` after provisioning.\n"]
    pub fn flow_logs_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_s3_bucket", self.base))
    }

    #[doc= "Get a reference to the value of field `flow_logs_s3_prefix` after provisioning.\n"]
    pub fn flow_logs_s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_s3_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorAcceleratorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl GlobalacceleratorAcceleratorTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for GlobalacceleratorAcceleratorTimeoutsEl {
    type O = BlockAssignable<GlobalacceleratorAcceleratorTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorAcceleratorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GlobalacceleratorAcceleratorTimeoutsElRef {
        GlobalacceleratorAcceleratorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorAcceleratorTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlobalacceleratorAcceleratorDynamic {
    attributes: Option<DynamicBlock<GlobalacceleratorAcceleratorAttributesEl>>,
}
