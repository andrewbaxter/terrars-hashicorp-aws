use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorklinkFleetData {
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
    audit_stream_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimize_for_end_user_location: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider: std::option::Option<std::vec::Vec<WorklinkFleetIdentityProviderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<std::vec::Vec<WorklinkFleetNetworkEl>>,
    dynamic: WorklinkFleetDynamic,
}

struct WorklinkFleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorklinkFleetData>,
}

#[derive(Clone)]
pub struct WorklinkFleet(std::rc::Rc<WorklinkFleet_>);

impl WorklinkFleet {
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

    #[doc = "Set the field `audit_stream_arn`.\n"]
    pub fn set_audit_stream_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().audit_stream_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_ca_certificate`.\n"]
    pub fn set_device_ca_certificate(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().device_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optimize_for_end_user_location`.\n"]
    pub fn set_optimize_for_end_user_location(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().optimize_for_end_user_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider`.\n"]
    pub fn set_identity_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorklinkFleetIdentityProviderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(self, v: impl std::convert::Into<terrars::BlockAssignable<WorklinkFleetNetworkEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_stream_arn` after provisioning.\n"]
    pub fn audit_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_stream_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_code` after provisioning.\n"]
    pub fn company_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_ca_certificate` after provisioning.\n"]
    pub fn device_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_ca_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `optimize_for_end_user_location` after provisioning.\n"]
    pub fn optimize_for_end_user_location(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.optimize_for_end_user_location", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> terrars::ListRef<WorklinkFleetIdentityProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<WorklinkFleetNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }
}

impl terrars::Referable for WorklinkFleet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorklinkFleet { }

impl terrars::ToListMappable for WorklinkFleet {
    type O = terrars::ListRef<WorklinkFleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorklinkFleet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_worklink_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorklinkFleet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorklinkFleet {
    pub fn build(self, stack: &mut terrars::Stack) -> WorklinkFleet {
        let out = WorklinkFleet(std::rc::Rc::new(WorklinkFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorklinkFleetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                audit_stream_arn: core::default::Default::default(),
                device_ca_certificate: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                optimize_for_end_user_location: core::default::Default::default(),
                identity_provider: core::default::Default::default(),
                network: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorklinkFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorklinkFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorklinkFleetRef {
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

    #[doc = "Get a reference to the value of field `audit_stream_arn` after provisioning.\n"]
    pub fn audit_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_stream_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_code` after provisioning.\n"]
    pub fn company_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_ca_certificate` after provisioning.\n"]
    pub fn device_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_ca_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `optimize_for_end_user_location` after provisioning.\n"]
    pub fn optimize_for_end_user_location(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.optimize_for_end_user_location", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> terrars::ListRef<WorklinkFleetIdentityProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<WorklinkFleetNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorklinkFleetIdentityProviderEl {
    saml_metadata: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WorklinkFleetIdentityProviderEl { }

impl terrars::ToListMappable for WorklinkFleetIdentityProviderEl {
    type O = terrars::BlockAssignable<WorklinkFleetIdentityProviderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorklinkFleetIdentityProviderEl {
    #[doc = ""]
    pub saml_metadata: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorklinkFleetIdentityProviderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorklinkFleetIdentityProviderElRef {
        WorklinkFleetIdentityProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorklinkFleetIdentityProviderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `saml_metadata` after provisioning.\n"]
    pub fn saml_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saml_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorklinkFleetNetworkEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl WorklinkFleetNetworkEl { }

impl terrars::ToListMappable for WorklinkFleetNetworkEl {
    type O = terrars::BlockAssignable<WorklinkFleetNetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorklinkFleetNetworkEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorklinkFleetNetworkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorklinkFleetNetworkElRef {
        WorklinkFleetNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorklinkFleetNetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorklinkFleetDynamic {
    identity_provider: std::option::Option<terrars::DynamicBlock<WorklinkFleetIdentityProviderEl>>,
    network: std::option::Option<terrars::DynamicBlock<WorklinkFleetNetworkEl>>,
}
