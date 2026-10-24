use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GrafanaWorkspaceSamlConfigurationData {
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
    admin_role_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_organizations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    editor_role_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_assertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups_assertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_metadata_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_metadata_xml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_assertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_validity_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_assertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org_assertion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_assertion: std::option::Option<terrars::PrimField<std::string::String>>,
    workspace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GrafanaWorkspaceSamlConfigurationTimeoutsEl>,
}

struct GrafanaWorkspaceSamlConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GrafanaWorkspaceSamlConfigurationData>,
}

#[derive(Clone)]
pub struct GrafanaWorkspaceSamlConfiguration(std::rc::Rc<GrafanaWorkspaceSamlConfiguration_>);

impl GrafanaWorkspaceSamlConfiguration {
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

    #[doc = "Set the field `admin_role_values`.\n"]
    pub fn set_admin_role_values(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().admin_role_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_organizations`.\n"]
    pub fn set_allowed_organizations(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_organizations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_assertion`.\n"]
    pub fn set_email_assertion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().email_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `groups_assertion`.\n"]
    pub fn set_groups_assertion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().groups_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idp_metadata_url`.\n"]
    pub fn set_idp_metadata_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().idp_metadata_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idp_metadata_xml`.\n"]
    pub fn set_idp_metadata_xml(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().idp_metadata_xml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_assertion`.\n"]
    pub fn set_login_assertion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().login_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_validity_duration`.\n"]
    pub fn set_login_validity_duration(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().login_validity_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_assertion`.\n"]
    pub fn set_name_assertion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `org_assertion`.\n"]
    pub fn set_org_assertion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().org_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_assertion`.\n"]
    pub fn set_role_assertion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GrafanaWorkspaceSamlConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `admin_role_values` after provisioning.\n"]
    pub fn admin_role_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_role_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_organizations` after provisioning.\n"]
    pub fn allowed_organizations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_organizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `editor_role_values` after provisioning.\n"]
    pub fn editor_role_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.editor_role_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_assertion` after provisioning.\n"]
    pub fn email_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `groups_assertion` after provisioning.\n"]
    pub fn groups_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.groups_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idp_metadata_url` after provisioning.\n"]
    pub fn idp_metadata_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\n"]
    pub fn idp_metadata_xml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_xml", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_assertion` after provisioning.\n"]
    pub fn login_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_validity_duration` after provisioning.\n"]
    pub fn login_validity_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_validity_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_assertion` after provisioning.\n"]
    pub fn name_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `org_assertion` after provisioning.\n"]
    pub fn org_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_assertion` after provisioning.\n"]
    pub fn role_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
        GrafanaWorkspaceSamlConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GrafanaWorkspaceSamlConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GrafanaWorkspaceSamlConfiguration { }

impl terrars::ToListMappable for GrafanaWorkspaceSamlConfiguration {
    type O = terrars::ListRef<GrafanaWorkspaceSamlConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GrafanaWorkspaceSamlConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_grafana_workspace_saml_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGrafanaWorkspaceSamlConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub editor_role_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub workspace_id: terrars::PrimField<std::string::String>,
}

impl BuildGrafanaWorkspaceSamlConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> GrafanaWorkspaceSamlConfiguration {
        let out = GrafanaWorkspaceSamlConfiguration(std::rc::Rc::new(GrafanaWorkspaceSamlConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GrafanaWorkspaceSamlConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_role_values: core::default::Default::default(),
                allowed_organizations: core::default::Default::default(),
                editor_role_values: self.editor_role_values,
                email_assertion: core::default::Default::default(),
                groups_assertion: core::default::Default::default(),
                id: core::default::Default::default(),
                idp_metadata_url: core::default::Default::default(),
                idp_metadata_xml: core::default::Default::default(),
                login_assertion: core::default::Default::default(),
                login_validity_duration: core::default::Default::default(),
                name_assertion: core::default::Default::default(),
                org_assertion: core::default::Default::default(),
                role_assertion: core::default::Default::default(),
                workspace_id: self.workspace_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GrafanaWorkspaceSamlConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GrafanaWorkspaceSamlConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GrafanaWorkspaceSamlConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_role_values` after provisioning.\n"]
    pub fn admin_role_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_role_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_organizations` after provisioning.\n"]
    pub fn allowed_organizations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_organizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `editor_role_values` after provisioning.\n"]
    pub fn editor_role_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.editor_role_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_assertion` after provisioning.\n"]
    pub fn email_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `groups_assertion` after provisioning.\n"]
    pub fn groups_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.groups_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idp_metadata_url` after provisioning.\n"]
    pub fn idp_metadata_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\n"]
    pub fn idp_metadata_xml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_metadata_xml", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_assertion` after provisioning.\n"]
    pub fn login_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `login_validity_duration` after provisioning.\n"]
    pub fn login_validity_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_validity_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_assertion` after provisioning.\n"]
    pub fn name_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `org_assertion` after provisioning.\n"]
    pub fn org_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_assertion` after provisioning.\n"]
    pub fn role_assertion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_assertion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
        GrafanaWorkspaceSamlConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GrafanaWorkspaceSamlConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GrafanaWorkspaceSamlConfigurationTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GrafanaWorkspaceSamlConfigurationTimeoutsEl {
    type O = terrars::BlockAssignable<GrafanaWorkspaceSamlConfigurationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGrafanaWorkspaceSamlConfigurationTimeoutsEl {}

impl BuildGrafanaWorkspaceSamlConfigurationTimeoutsEl {
    pub fn build(self) -> GrafanaWorkspaceSamlConfigurationTimeoutsEl {
        GrafanaWorkspaceSamlConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
        GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
