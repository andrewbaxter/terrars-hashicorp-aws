use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DirectoryServiceRadiusSettingsData {
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
    authentication_protocol: terrars::PrimField<std::string::String>,
    directory_id: terrars::PrimField<std::string::String>,
    display_label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    radius_port: terrars::PrimField<f64>,
    radius_retries: terrars::PrimField<f64>,
    radius_servers: terrars::SetField<terrars::PrimField<std::string::String>>,
    radius_timeout: terrars::PrimField<f64>,
    shared_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_same_username: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DirectoryServiceRadiusSettingsTimeoutsEl>,
}

struct DirectoryServiceRadiusSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DirectoryServiceRadiusSettingsData>,
}

#[derive(Clone)]
pub struct DirectoryServiceRadiusSettings(std::rc::Rc<DirectoryServiceRadiusSettings_>);

impl DirectoryServiceRadiusSettings {
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

    #[doc = "Set the field `use_same_username`.\n"]
    pub fn set_use_same_username(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_same_username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DirectoryServiceRadiusSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `authentication_protocol` after provisioning.\n"]
    pub fn authentication_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_label` after provisioning.\n"]
    pub fn display_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_port` after provisioning.\n"]
    pub fn radius_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_retries` after provisioning.\n"]
    pub fn radius_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_servers` after provisioning.\n"]
    pub fn radius_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.radius_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_timeout` after provisioning.\n"]
    pub fn radius_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_secret` after provisioning.\n"]
    pub fn shared_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_same_username` after provisioning.\n"]
    pub fn use_same_username(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_same_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceRadiusSettingsTimeoutsElRef {
        DirectoryServiceRadiusSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DirectoryServiceRadiusSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DirectoryServiceRadiusSettings { }

impl terrars::ToListMappable for DirectoryServiceRadiusSettings {
    type O = terrars::ListRef<DirectoryServiceRadiusSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DirectoryServiceRadiusSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_directory_service_radius_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceRadiusSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub authentication_protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub directory_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub display_label: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub radius_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub radius_retries: terrars::PrimField<f64>,
    #[doc = ""]
    pub radius_servers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub radius_timeout: terrars::PrimField<f64>,
    #[doc = ""]
    pub shared_secret: terrars::PrimField<std::string::String>,
}

impl BuildDirectoryServiceRadiusSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DirectoryServiceRadiusSettings {
        let out = DirectoryServiceRadiusSettings(std::rc::Rc::new(DirectoryServiceRadiusSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DirectoryServiceRadiusSettingsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authentication_protocol: self.authentication_protocol,
                directory_id: self.directory_id,
                display_label: self.display_label,
                id: core::default::Default::default(),
                radius_port: self.radius_port,
                radius_retries: self.radius_retries,
                radius_servers: self.radius_servers,
                radius_timeout: self.radius_timeout,
                shared_secret: self.shared_secret,
                use_same_username: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DirectoryServiceRadiusSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceRadiusSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DirectoryServiceRadiusSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_protocol` after provisioning.\n"]
    pub fn authentication_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_label` after provisioning.\n"]
    pub fn display_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_port` after provisioning.\n"]
    pub fn radius_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_retries` after provisioning.\n"]
    pub fn radius_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_servers` after provisioning.\n"]
    pub fn radius_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.radius_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_timeout` after provisioning.\n"]
    pub fn radius_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_secret` after provisioning.\n"]
    pub fn shared_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_same_username` after provisioning.\n"]
    pub fn use_same_username(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_same_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceRadiusSettingsTimeoutsElRef {
        DirectoryServiceRadiusSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DirectoryServiceRadiusSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DirectoryServiceRadiusSettingsTimeoutsEl {
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

impl terrars::ToListMappable for DirectoryServiceRadiusSettingsTimeoutsEl {
    type O = terrars::BlockAssignable<DirectoryServiceRadiusSettingsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceRadiusSettingsTimeoutsEl {}

impl BuildDirectoryServiceRadiusSettingsTimeoutsEl {
    pub fn build(self) -> DirectoryServiceRadiusSettingsTimeoutsEl {
        DirectoryServiceRadiusSettingsTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DirectoryServiceRadiusSettingsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceRadiusSettingsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DirectoryServiceRadiusSettingsTimeoutsElRef {
        DirectoryServiceRadiusSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceRadiusSettingsTimeoutsElRef {
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
