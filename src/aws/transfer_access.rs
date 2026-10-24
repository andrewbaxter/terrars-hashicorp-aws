use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferAccessData {
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
    external_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    home_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    home_directory_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    server_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    home_directory_mappings: std::option::Option<std::vec::Vec<TransferAccessHomeDirectoryMappingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_profile: std::option::Option<std::vec::Vec<TransferAccessPosixProfileEl>>,
    dynamic: TransferAccessDynamic,
}

struct TransferAccess_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TransferAccessData>,
}

#[derive(Clone)]
pub struct TransferAccess(std::rc::Rc<TransferAccess_>);

impl TransferAccess {
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

    #[doc = "Set the field `home_directory`.\n"]
    pub fn set_home_directory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().home_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `home_directory_type`.\n"]
    pub fn set_home_directory_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().home_directory_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `home_directory_mappings`.\n"]
    pub fn set_home_directory_mappings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferAccessHomeDirectoryMappingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().home_directory_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.home_directory_mappings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `posix_profile`.\n"]
    pub fn set_posix_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferAccessPosixProfileEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().posix_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.posix_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_directory` after provisioning.\n"]
    pub fn home_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_directory_type` after provisioning.\n"]
    pub fn home_directory_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_id` after provisioning.\n"]
    pub fn server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_directory_mappings` after provisioning.\n"]
    pub fn home_directory_mappings(&self) -> terrars::ListRef<TransferAccessHomeDirectoryMappingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.home_directory_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_profile` after provisioning.\n"]
    pub fn posix_profile(&self) -> terrars::ListRef<TransferAccessPosixProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_profile", self.extract_ref()))
    }
}

impl terrars::Referable for TransferAccess {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TransferAccess { }

impl terrars::ToListMappable for TransferAccess {
    type O = terrars::ListRef<TransferAccessRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TransferAccess_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_transfer_access".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferAccess {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub external_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub server_id: terrars::PrimField<std::string::String>,
}

impl BuildTransferAccess {
    pub fn build(self, stack: &mut terrars::Stack) -> TransferAccess {
        let out = TransferAccess(std::rc::Rc::new(TransferAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TransferAccessData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                external_id: self.external_id,
                home_directory: core::default::Default::default(),
                home_directory_type: core::default::Default::default(),
                id: core::default::Default::default(),
                policy: core::default::Default::default(),
                role: core::default::Default::default(),
                server_id: self.server_id,
                home_directory_mappings: core::default::Default::default(),
                posix_profile: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TransferAccessRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_directory` after provisioning.\n"]
    pub fn home_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_directory_type` after provisioning.\n"]
    pub fn home_directory_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_id` after provisioning.\n"]
    pub fn server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_directory_mappings` after provisioning.\n"]
    pub fn home_directory_mappings(&self) -> terrars::ListRef<TransferAccessHomeDirectoryMappingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.home_directory_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_profile` after provisioning.\n"]
    pub fn posix_profile(&self) -> terrars::ListRef<TransferAccessPosixProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_profile", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TransferAccessHomeDirectoryMappingsEl {
    entry: terrars::PrimField<std::string::String>,
    target: terrars::PrimField<std::string::String>,
}

impl TransferAccessHomeDirectoryMappingsEl { }

impl terrars::ToListMappable for TransferAccessHomeDirectoryMappingsEl {
    type O = terrars::BlockAssignable<TransferAccessHomeDirectoryMappingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferAccessHomeDirectoryMappingsEl {
    #[doc = ""]
    pub entry: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildTransferAccessHomeDirectoryMappingsEl {
    pub fn build(self) -> TransferAccessHomeDirectoryMappingsEl {
        TransferAccessHomeDirectoryMappingsEl {
            entry: self.entry,
            target: self.target,
        }
    }
}

pub struct TransferAccessHomeDirectoryMappingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferAccessHomeDirectoryMappingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferAccessHomeDirectoryMappingsElRef {
        TransferAccessHomeDirectoryMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferAccessHomeDirectoryMappingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferAccessPosixProfileEl {
    gid: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_gids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    uid: terrars::PrimField<f64>,
}

impl TransferAccessPosixProfileEl {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.secondary_gids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferAccessPosixProfileEl {
    type O = terrars::BlockAssignable<TransferAccessPosixProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferAccessPosixProfileEl {
    #[doc = ""]
    pub gid: terrars::PrimField<f64>,
    #[doc = ""]
    pub uid: terrars::PrimField<f64>,
}

impl BuildTransferAccessPosixProfileEl {
    pub fn build(self) -> TransferAccessPosixProfileEl {
        TransferAccessPosixProfileEl {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}

pub struct TransferAccessPosixProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferAccessPosixProfileElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferAccessPosixProfileElRef {
        TransferAccessPosixProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferAccessPosixProfileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_gids` after provisioning.\n"]
    pub fn secondary_gids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_gids", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferAccessDynamic {
    home_directory_mappings: std::option::Option<terrars::DynamicBlock<TransferAccessHomeDirectoryMappingsEl>>,
    posix_profile: std::option::Option<terrars::DynamicBlock<TransferAccessPosixProfileEl>>,
}
