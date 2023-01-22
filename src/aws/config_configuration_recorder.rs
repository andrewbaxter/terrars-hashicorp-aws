use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConfigConfigurationRecorderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_group: Option<Vec<ConfigConfigurationRecorderRecordingGroupEl>>,
    dynamic: ConfigConfigurationRecorderDynamic,
}

struct ConfigConfigurationRecorder_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigConfigurationRecorderData>,
}

#[derive(Clone)]
pub struct ConfigConfigurationRecorder(Rc<ConfigConfigurationRecorder_>);

impl ConfigConfigurationRecorder {
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

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `recording_group`.\n"]
    pub fn set_recording_group(
        self,
        v: impl Into<BlockAssignable<ConfigConfigurationRecorderRecordingGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording_group = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recording_group` after provisioning.\n"]
    pub fn recording_group(&self) -> ListRef<ConfigConfigurationRecorderRecordingGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recording_group", self.extract_ref()))
    }
}

impl Resource for ConfigConfigurationRecorder {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for ConfigConfigurationRecorder {
    type O = ListRef<ConfigConfigurationRecorderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConfigConfigurationRecorder_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_configuration_recorder".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigConfigurationRecorder {
    pub tf_id: String,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildConfigConfigurationRecorder {
    pub fn build(self, stack: &mut Stack) -> ConfigConfigurationRecorder {
        let out = ConfigConfigurationRecorder(Rc::new(ConfigConfigurationRecorder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigConfigurationRecorderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                role_arn: self.role_arn,
                recording_group: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigConfigurationRecorderRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigConfigurationRecorderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConfigConfigurationRecorderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recording_group` after provisioning.\n"]
    pub fn recording_group(&self) -> ListRef<ConfigConfigurationRecorderRecordingGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recording_group", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigurationRecorderRecordingGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_supported: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_global_resource_types: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types: Option<SetField<PrimField<String>>>,
}

impl ConfigConfigurationRecorderRecordingGroupEl {
    #[doc= "Set the field `all_supported`.\n"]
    pub fn set_all_supported(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_supported = Some(v.into());
        self
    }

    #[doc= "Set the field `include_global_resource_types`.\n"]
    pub fn set_include_global_resource_types(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_global_resource_types = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_types`.\n"]
    pub fn set_resource_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resource_types = Some(v.into());
        self
    }
}

impl ToListMappable for ConfigConfigurationRecorderRecordingGroupEl {
    type O = BlockAssignable<ConfigConfigurationRecorderRecordingGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationRecorderRecordingGroupEl {}

impl BuildConfigConfigurationRecorderRecordingGroupEl {
    pub fn build(self) -> ConfigConfigurationRecorderRecordingGroupEl {
        ConfigConfigurationRecorderRecordingGroupEl {
            all_supported: core::default::Default::default(),
            include_global_resource_types: core::default::Default::default(),
            resource_types: core::default::Default::default(),
        }
    }
}

pub struct ConfigConfigurationRecorderRecordingGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigConfigurationRecorderRecordingGroupElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigurationRecorderRecordingGroupElRef {
        ConfigConfigurationRecorderRecordingGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationRecorderRecordingGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `all_supported` after provisioning.\n"]
    pub fn all_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_supported", self.base))
    }

    #[doc= "Get a reference to the value of field `include_global_resource_types` after provisioning.\n"]
    pub fn include_global_resource_types(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_global_resource_types", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigurationRecorderDynamic {
    recording_group: Option<DynamicBlock<ConfigConfigurationRecorderRecordingGroupEl>>,
}
