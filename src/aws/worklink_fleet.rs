use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorklinkFleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_stream_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optimize_for_end_user_location: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider: Option<Vec<WorklinkFleetIdentityProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Vec<WorklinkFleetNetworkEl>>,
    dynamic: WorklinkFleetDynamic,
}

struct WorklinkFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorklinkFleetData>,
}

#[derive(Clone)]
pub struct WorklinkFleet(Rc<WorklinkFleet_>);

impl WorklinkFleet {
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

    #[doc= "Set the field `audit_stream_arn`.\n"]
    pub fn set_audit_stream_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().audit_stream_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `device_ca_certificate`.\n"]
    pub fn set_device_ca_certificate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().device_ca_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `optimize_for_end_user_location`.\n"]
    pub fn set_optimize_for_end_user_location(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().optimize_for_end_user_location = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider`.\n"]
    pub fn set_identity_provider(self, v: impl Into<BlockAssignable<WorklinkFleetIdentityProviderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_provider = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_provider = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network`.\n"]
    pub fn set_network(self, v: impl Into<BlockAssignable<WorklinkFleetNetworkEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `audit_stream_arn` after provisioning.\n"]
    pub fn audit_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audit_stream_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `company_code` after provisioning.\n"]
    pub fn company_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_ca_certificate` after provisioning.\n"]
    pub fn device_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_ca_certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `optimize_for_end_user_location` after provisioning.\n"]
    pub fn optimize_for_end_user_location(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.optimize_for_end_user_location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> ListRef<WorklinkFleetIdentityProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> ListRef<WorklinkFleetNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }
}

impl Referable for WorklinkFleet {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WorklinkFleet { }

impl ToListMappable for WorklinkFleet {
    type O = ListRef<WorklinkFleetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WorklinkFleet_ {
    fn extract_resource_type(&self) -> String {
        "aws_worklink_fleet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorklinkFleet {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildWorklinkFleet {
    pub fn build(self, stack: &mut Stack) -> WorklinkFleet {
        let out = WorklinkFleet(Rc::new(WorklinkFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorklinkFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                audit_stream_arn: core::default::Default::default(),
                device_ca_certificate: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                optimize_for_end_user_location: core::default::Default::default(),
                identity_provider: core::default::Default::default(),
                network: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorklinkFleetRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorklinkFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorklinkFleetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `audit_stream_arn` after provisioning.\n"]
    pub fn audit_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audit_stream_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `company_code` after provisioning.\n"]
    pub fn company_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_ca_certificate` after provisioning.\n"]
    pub fn device_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_ca_certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `optimize_for_end_user_location` after provisioning.\n"]
    pub fn optimize_for_end_user_location(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.optimize_for_end_user_location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> ListRef<WorklinkFleetIdentityProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> ListRef<WorklinkFleetNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorklinkFleetIdentityProviderEl {
    saml_metadata: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WorklinkFleetIdentityProviderEl { }

impl ToListMappable for WorklinkFleetIdentityProviderEl {
    type O = BlockAssignable<WorklinkFleetIdentityProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorklinkFleetIdentityProviderEl {
    #[doc= ""]
    pub saml_metadata: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildWorklinkFleetIdentityProviderEl {
    pub fn build(self) -> WorklinkFleetIdentityProviderEl {
        WorklinkFleetIdentityProviderEl {
            saml_metadata: self.saml_metadata,
            type_: self.type_,
        }
    }
}

pub struct WorklinkFleetIdentityProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorklinkFleetIdentityProviderElRef {
    fn new(shared: StackShared, base: String) -> WorklinkFleetIdentityProviderElRef {
        WorklinkFleetIdentityProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorklinkFleetIdentityProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `saml_metadata` after provisioning.\n"]
    pub fn saml_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_metadata", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorklinkFleetNetworkEl {
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}

impl WorklinkFleetNetworkEl { }

impl ToListMappable for WorklinkFleetNetworkEl {
    type O = BlockAssignable<WorklinkFleetNetworkEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorklinkFleetNetworkEl {
    #[doc= ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc= ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc= ""]
    pub vpc_id: PrimField<String>,
}

impl BuildWorklinkFleetNetworkEl {
    pub fn build(self) -> WorklinkFleetNetworkEl {
        WorklinkFleetNetworkEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct WorklinkFleetNetworkElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorklinkFleetNetworkElRef {
    fn new(shared: StackShared, base: String) -> WorklinkFleetNetworkElRef {
        WorklinkFleetNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorklinkFleetNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorklinkFleetDynamic {
    identity_provider: Option<DynamicBlock<WorklinkFleetIdentityProviderEl>>,
    network: Option<DynamicBlock<WorklinkFleetNetworkEl>>,
}
