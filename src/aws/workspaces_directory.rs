use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspacesDirectoryData {
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
    directory_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_service_permissions: std::option::Option<std::vec::Vec<WorkspacesDirectorySelfServicePermissionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_access_properties: std::option::Option<std::vec::Vec<WorkspacesDirectoryWorkspaceAccessPropertiesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_creation_properties: std::option::Option<std::vec::Vec<WorkspacesDirectoryWorkspaceCreationPropertiesEl>>,
    dynamic: WorkspacesDirectoryDynamic,
}

struct WorkspacesDirectory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkspacesDirectoryData>,
}

#[derive(Clone)]
pub struct WorkspacesDirectory(std::rc::Rc<WorkspacesDirectory_>);

impl WorkspacesDirectory {
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

    #[doc = "Set the field `ip_group_ids`.\n"]
    pub fn set_ip_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ip_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subnet_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `self_service_permissions`.\n"]
    pub fn set_self_service_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspacesDirectorySelfServicePermissionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_service_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_service_permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_access_properties`.\n"]
    pub fn set_workspace_access_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspacesDirectoryWorkspaceAccessPropertiesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workspace_access_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workspace_access_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_creation_properties`.\n"]
    pub fn set_workspace_creation_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspacesDirectoryWorkspaceCreationPropertiesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workspace_creation_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workspace_creation_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(&self) -> terrars::ListRef<WorkspacesDirectorySelfServicePermissionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_service_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(&self) -> terrars::ListRef<WorkspacesDirectoryWorkspaceAccessPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_access_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(&self) -> terrars::ListRef<WorkspacesDirectoryWorkspaceCreationPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_creation_properties", self.extract_ref()))
    }
}

impl terrars::Referable for WorkspacesDirectory {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkspacesDirectory { }

impl terrars::ToListMappable for WorkspacesDirectory {
    type O = terrars::ListRef<WorkspacesDirectoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkspacesDirectory_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_workspaces_directory".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspacesDirectory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub directory_id: terrars::PrimField<std::string::String>,
}

impl BuildWorkspacesDirectory {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkspacesDirectory {
        let out = WorkspacesDirectory(std::rc::Rc::new(WorkspacesDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkspacesDirectoryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                ip_group_ids: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                self_service_permissions: core::default::Default::default(),
                workspace_access_properties: core::default::Default::default(),
                workspace_creation_properties: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspacesDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkspacesDirectoryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(&self) -> terrars::ListRef<WorkspacesDirectorySelfServicePermissionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_service_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(&self) -> terrars::ListRef<WorkspacesDirectoryWorkspaceAccessPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_access_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(&self) -> terrars::ListRef<WorkspacesDirectoryWorkspaceCreationPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_creation_properties", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkspacesDirectorySelfServicePermissionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    change_compute_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    increase_volume_size: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rebuild_workspace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_workspace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    switch_running_mode: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkspacesDirectorySelfServicePermissionsEl {
    #[doc = "Set the field `change_compute_type`.\n"]
    pub fn set_change_compute_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.change_compute_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `increase_volume_size`.\n"]
    pub fn set_increase_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.increase_volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rebuild_workspace`.\n"]
    pub fn set_rebuild_workspace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rebuild_workspace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_workspace`.\n"]
    pub fn set_restart_workspace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.restart_workspace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `switch_running_mode`.\n"]
    pub fn set_switch_running_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.switch_running_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspacesDirectorySelfServicePermissionsEl {
    type O = terrars::BlockAssignable<WorkspacesDirectorySelfServicePermissionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesDirectorySelfServicePermissionsEl {}

impl BuildWorkspacesDirectorySelfServicePermissionsEl {
    pub fn build(self) -> WorkspacesDirectorySelfServicePermissionsEl {
        WorkspacesDirectorySelfServicePermissionsEl {
            change_compute_type: core::default::Default::default(),
            increase_volume_size: core::default::Default::default(),
            rebuild_workspace: core::default::Default::default(),
            restart_workspace: core::default::Default::default(),
            switch_running_mode: core::default::Default::default(),
        }
    }
}

pub struct WorkspacesDirectorySelfServicePermissionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesDirectorySelfServicePermissionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspacesDirectorySelfServicePermissionsElRef {
        WorkspacesDirectorySelfServicePermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesDirectorySelfServicePermissionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `change_compute_type` after provisioning.\n"]
    pub fn change_compute_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.change_compute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `increase_volume_size` after provisioning.\n"]
    pub fn increase_volume_size(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.increase_volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `rebuild_workspace` after provisioning.\n"]
    pub fn rebuild_workspace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rebuild_workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_workspace` after provisioning.\n"]
    pub fn restart_workspace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `switch_running_mode` after provisioning.\n"]
    pub fn switch_running_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.switch_running_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspacesDirectoryWorkspaceAccessPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_android: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_chromeos: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_ios: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_linux: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_osx: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_web: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_windows: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_zeroclient: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkspacesDirectoryWorkspaceAccessPropertiesEl {
    #[doc = "Set the field `device_type_android`.\n"]
    pub fn set_device_type_android(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_android = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_chromeos`.\n"]
    pub fn set_device_type_chromeos(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_chromeos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_ios`.\n"]
    pub fn set_device_type_ios(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_ios = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_linux`.\n"]
    pub fn set_device_type_linux(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_linux = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_osx`.\n"]
    pub fn set_device_type_osx(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_osx = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_web`.\n"]
    pub fn set_device_type_web(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_web = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_windows`.\n"]
    pub fn set_device_type_windows(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_windows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_zeroclient`.\n"]
    pub fn set_device_type_zeroclient(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_zeroclient = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspacesDirectoryWorkspaceAccessPropertiesEl {
    type O = terrars::BlockAssignable<WorkspacesDirectoryWorkspaceAccessPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesDirectoryWorkspaceAccessPropertiesEl {}

impl BuildWorkspacesDirectoryWorkspaceAccessPropertiesEl {
    pub fn build(self) -> WorkspacesDirectoryWorkspaceAccessPropertiesEl {
        WorkspacesDirectoryWorkspaceAccessPropertiesEl {
            device_type_android: core::default::Default::default(),
            device_type_chromeos: core::default::Default::default(),
            device_type_ios: core::default::Default::default(),
            device_type_linux: core::default::Default::default(),
            device_type_osx: core::default::Default::default(),
            device_type_web: core::default::Default::default(),
            device_type_windows: core::default::Default::default(),
            device_type_zeroclient: core::default::Default::default(),
        }
    }
}

pub struct WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
        WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_type_android` after provisioning.\n"]
    pub fn device_type_android(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_android", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_chromeos` after provisioning.\n"]
    pub fn device_type_chromeos(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_chromeos", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_ios` after provisioning.\n"]
    pub fn device_type_ios(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_ios", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_linux` after provisioning.\n"]
    pub fn device_type_linux(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_linux", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_osx` after provisioning.\n"]
    pub fn device_type_osx(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_osx", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_web` after provisioning.\n"]
    pub fn device_type_web(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_web", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_windows` after provisioning.\n"]
    pub fn device_type_windows(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_windows", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_zeroclient` after provisioning.\n"]
    pub fn device_type_zeroclient(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_zeroclient", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspacesDirectoryWorkspaceCreationPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ou: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_internet_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_maintenance_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_enabled_as_local_administrator: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkspacesDirectoryWorkspaceCreationPropertiesEl {
    #[doc = "Set the field `custom_security_group_id`.\n"]
    pub fn set_custom_security_group_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_security_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_ou`.\n"]
    pub fn set_default_ou(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_ou = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_internet_access`.\n"]
    pub fn set_enable_internet_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_internet_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_maintenance_mode`.\n"]
    pub fn set_enable_maintenance_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_maintenance_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_enabled_as_local_administrator`.\n"]
    pub fn set_user_enabled_as_local_administrator(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.user_enabled_as_local_administrator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspacesDirectoryWorkspaceCreationPropertiesEl {
    type O = terrars::BlockAssignable<WorkspacesDirectoryWorkspaceCreationPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspacesDirectoryWorkspaceCreationPropertiesEl {}

impl BuildWorkspacesDirectoryWorkspaceCreationPropertiesEl {
    pub fn build(self) -> WorkspacesDirectoryWorkspaceCreationPropertiesEl {
        WorkspacesDirectoryWorkspaceCreationPropertiesEl {
            custom_security_group_id: core::default::Default::default(),
            default_ou: core::default::Default::default(),
            enable_internet_access: core::default::Default::default(),
            enable_maintenance_mode: core::default::Default::default(),
            user_enabled_as_local_administrator: core::default::Default::default(),
        }
    }
}

pub struct WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
        WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_security_group_id` after provisioning.\n"]
    pub fn custom_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ou` after provisioning.\n"]
    pub fn default_ou(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ou", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_internet_access` after provisioning.\n"]
    pub fn enable_internet_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_internet_access", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_maintenance_mode` after provisioning.\n"]
    pub fn enable_maintenance_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_maintenance_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `user_enabled_as_local_administrator` after provisioning.\n"]
    pub fn user_enabled_as_local_administrator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_enabled_as_local_administrator", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspacesDirectoryDynamic {
    self_service_permissions: std::option::Option<terrars::DynamicBlock<WorkspacesDirectorySelfServicePermissionsEl>>,
    workspace_access_properties: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspacesDirectoryWorkspaceAccessPropertiesEl>>,
    workspace_creation_properties: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspacesDirectoryWorkspaceCreationPropertiesEl>>,
}
