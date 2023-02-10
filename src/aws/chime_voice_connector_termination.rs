use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ChimeVoiceConnectorTerminationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    calling_regions: SetField<PrimField<String>>,
    cidr_allow_list: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cps_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    voice_connector_id: PrimField<String>,
}

struct ChimeVoiceConnectorTermination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChimeVoiceConnectorTerminationData>,
}

#[derive(Clone)]
pub struct ChimeVoiceConnectorTermination(Rc<ChimeVoiceConnectorTermination_>);

impl ChimeVoiceConnectorTermination {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `cps_limit`.\n"]
    pub fn set_cps_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cps_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `default_phone_number`.\n"]
    pub fn set_default_phone_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_phone_number = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `calling_regions` after provisioning.\n"]
    pub fn calling_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.calling_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cidr_allow_list` after provisioning.\n"]
    pub fn cidr_allow_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidr_allow_list", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cps_limit` after provisioning.\n"]
    pub fn cps_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cps_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_phone_number` after provisioning.\n"]
    pub fn default_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_phone_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice_connector_id", self.extract_ref()))
    }
}

impl Resource for ChimeVoiceConnectorTermination {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ChimeVoiceConnectorTermination {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ChimeVoiceConnectorTermination {
    type O = ListRef<ChimeVoiceConnectorTerminationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for ChimeVoiceConnectorTermination_ {
    fn extract_resource_type(&self) -> String {
        "aws_chime_voice_connector_termination".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChimeVoiceConnectorTermination {
    pub tf_id: String,
    #[doc= ""]
    pub calling_regions: SetField<PrimField<String>>,
    #[doc= ""]
    pub cidr_allow_list: SetField<PrimField<String>>,
    #[doc= ""]
    pub voice_connector_id: PrimField<String>,
}

impl BuildChimeVoiceConnectorTermination {
    pub fn build(self, stack: &mut Stack) -> ChimeVoiceConnectorTermination {
        let out = ChimeVoiceConnectorTermination(Rc::new(ChimeVoiceConnectorTermination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChimeVoiceConnectorTerminationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                calling_regions: self.calling_regions,
                cidr_allow_list: self.cidr_allow_list,
                cps_limit: core::default::Default::default(),
                default_phone_number: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                voice_connector_id: self.voice_connector_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChimeVoiceConnectorTerminationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ChimeVoiceConnectorTerminationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChimeVoiceConnectorTerminationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `calling_regions` after provisioning.\n"]
    pub fn calling_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.calling_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cidr_allow_list` after provisioning.\n"]
    pub fn cidr_allow_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidr_allow_list", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cps_limit` after provisioning.\n"]
    pub fn cps_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cps_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_phone_number` after provisioning.\n"]
    pub fn default_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_phone_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice_connector_id", self.extract_ref()))
    }
}
