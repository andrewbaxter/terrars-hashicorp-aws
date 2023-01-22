use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDeviceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    device_fleet_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<Vec<SagemakerDeviceDeviceEl>>,
    dynamic: SagemakerDeviceDynamic,
}

struct SagemakerDevice_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerDeviceData>,
}

#[derive(Clone)]
pub struct SagemakerDevice(Rc<SagemakerDevice_>);

impl SagemakerDevice {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `device`.\n"]
    pub fn set_device(self, v: impl Into<BlockAssignable<SagemakerDeviceDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_fleet_name` after provisioning.\n"]
    pub fn device_fleet_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_fleet_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> ListRef<SagemakerDeviceDeviceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.device", self.extract_ref()))
    }
}

impl Resource for SagemakerDevice {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for SagemakerDevice {
    type O = ListRef<SagemakerDeviceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerDevice_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_device".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerDevice {
    pub tf_id: String,
    #[doc= ""]
    pub device_fleet_name: PrimField<String>,
}

impl BuildSagemakerDevice {
    pub fn build(self, stack: &mut Stack) -> SagemakerDevice {
        let out = SagemakerDevice(Rc::new(SagemakerDevice_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerDeviceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                device_fleet_name: self.device_fleet_name,
                id: core::default::Default::default(),
                device: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerDeviceRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDeviceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerDeviceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_fleet_name` after provisioning.\n"]
    pub fn device_fleet_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_fleet_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> ListRef<SagemakerDeviceDeviceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.device", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerDeviceDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iot_thing_name: Option<PrimField<String>>,
}

impl SagemakerDeviceDeviceEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `iot_thing_name`.\n"]
    pub fn set_iot_thing_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iot_thing_name = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDeviceDeviceEl {
    type O = BlockAssignable<SagemakerDeviceDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDeviceDeviceEl {
    #[doc= ""]
    pub device_name: PrimField<String>,
}

impl BuildSagemakerDeviceDeviceEl {
    pub fn build(self) -> SagemakerDeviceDeviceEl {
        SagemakerDeviceDeviceEl {
            description: core::default::Default::default(),
            device_name: self.device_name,
            iot_thing_name: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDeviceDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDeviceDeviceElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDeviceDeviceElRef {
        SagemakerDeviceDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDeviceDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `iot_thing_name` after provisioning.\n"]
    pub fn iot_thing_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iot_thing_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDeviceDynamic {
    device: Option<DynamicBlock<SagemakerDeviceDeviceEl>>,
}