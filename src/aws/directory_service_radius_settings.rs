use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DirectoryServiceRadiusSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    authentication_protocol: PrimField<String>,
    directory_id: PrimField<String>,
    display_label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    radius_port: PrimField<f64>,
    radius_retries: PrimField<f64>,
    radius_servers: SetField<PrimField<String>>,
    radius_timeout: PrimField<f64>,
    shared_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_same_username: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DirectoryServiceRadiusSettingsTimeoutsEl>,
}

struct DirectoryServiceRadiusSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DirectoryServiceRadiusSettingsData>,
}

#[derive(Clone)]
pub struct DirectoryServiceRadiusSettings(Rc<DirectoryServiceRadiusSettings_>);

impl DirectoryServiceRadiusSettings {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `use_same_username`.\n"]
    pub fn set_use_same_username(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_same_username = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DirectoryServiceRadiusSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `authentication_protocol` after provisioning.\n"]
    pub fn authentication_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_label` after provisioning.\n"]
    pub fn display_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_port` after provisioning.\n"]
    pub fn radius_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_retries` after provisioning.\n"]
    pub fn radius_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_retries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_servers` after provisioning.\n"]
    pub fn radius_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.radius_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_timeout` after provisioning.\n"]
    pub fn radius_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_secret` after provisioning.\n"]
    pub fn shared_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_same_username` after provisioning.\n"]
    pub fn use_same_username(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_same_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceRadiusSettingsTimeoutsElRef {
        DirectoryServiceRadiusSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Resource for DirectoryServiceRadiusSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DirectoryServiceRadiusSettings {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for DirectoryServiceRadiusSettings {
    type O = ListRef<DirectoryServiceRadiusSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for DirectoryServiceRadiusSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_directory_service_radius_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceRadiusSettings {
    pub tf_id: String,
    #[doc= ""]
    pub authentication_protocol: PrimField<String>,
    #[doc= ""]
    pub directory_id: PrimField<String>,
    #[doc= ""]
    pub display_label: PrimField<String>,
    #[doc= ""]
    pub radius_port: PrimField<f64>,
    #[doc= ""]
    pub radius_retries: PrimField<f64>,
    #[doc= ""]
    pub radius_servers: SetField<PrimField<String>>,
    #[doc= ""]
    pub radius_timeout: PrimField<f64>,
    #[doc= ""]
    pub shared_secret: PrimField<String>,
}

impl BuildDirectoryServiceRadiusSettings {
    pub fn build(self, stack: &mut Stack) -> DirectoryServiceRadiusSettings {
        let out = DirectoryServiceRadiusSettings(Rc::new(DirectoryServiceRadiusSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DirectoryServiceRadiusSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceRadiusSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DirectoryServiceRadiusSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `authentication_protocol` after provisioning.\n"]
    pub fn authentication_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_label` after provisioning.\n"]
    pub fn display_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_port` after provisioning.\n"]
    pub fn radius_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_retries` after provisioning.\n"]
    pub fn radius_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_retries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_servers` after provisioning.\n"]
    pub fn radius_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.radius_servers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `radius_timeout` after provisioning.\n"]
    pub fn radius_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_secret` after provisioning.\n"]
    pub fn shared_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_same_username` after provisioning.\n"]
    pub fn use_same_username(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_same_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceRadiusSettingsTimeoutsElRef {
        DirectoryServiceRadiusSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DirectoryServiceRadiusSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DirectoryServiceRadiusSettingsTimeoutsEl {
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

impl ToListMappable for DirectoryServiceRadiusSettingsTimeoutsEl {
    type O = BlockAssignable<DirectoryServiceRadiusSettingsTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceRadiusSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DirectoryServiceRadiusSettingsTimeoutsElRef {
        DirectoryServiceRadiusSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceRadiusSettingsTimeoutsElRef {
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
