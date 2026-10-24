use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailDatabaseData {
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
    apply_immediately: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_enabled: std::option::Option<terrars::PrimField<bool>>,
    blueprint_id: terrars::PrimField<std::string::String>,
    bundle_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_snapshot_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    master_database_name: terrars::PrimField<std::string::String>,
    master_password: terrars::PrimField<std::string::String>,
    master_username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_backup_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_maintenance_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
    relational_database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_final_snapshot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct LightsailDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LightsailDatabaseData>,
}

#[derive(Clone)]
pub struct LightsailDatabase(std::rc::Rc<LightsailDatabase_>);

impl LightsailDatabase {
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

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_enabled`.\n"]
    pub fn set_backup_retention_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().backup_retention_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `final_snapshot_name`.\n"]
    pub fn set_final_snapshot_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().final_snapshot_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_backup_window`.\n"]
    pub fn set_preferred_backup_window(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_backup_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_maintenance_window`.\n"]
    pub fn set_preferred_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_final_snapshot`.\n"]
    pub fn set_skip_final_snapshot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_snapshot = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_enabled` after provisioning.\n"]
    pub fn backup_retention_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blueprint_id` after provisioning.\n"]
    pub fn blueprint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blueprint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_identifier` after provisioning.\n"]
    pub fn ca_certificate_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_name` after provisioning.\n"]
    pub fn final_snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_database_name` after provisioning.\n"]
    pub fn master_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_address` after provisioning.\n"]
    pub fn master_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_port` after provisioning.\n"]
    pub fn master_endpoint_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_size` after provisioning.\n"]
    pub fn ram_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ram_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_name` after provisioning.\n"]
    pub fn relational_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relational_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_availability_zone` after provisioning.\n"]
    pub fn secondary_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `support_code` after provisioning.\n"]
    pub fn support_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for LightsailDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LightsailDatabase { }

impl terrars::ToListMappable for LightsailDatabase {
    type O = terrars::ListRef<LightsailDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LightsailDatabase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lightsail_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailDatabase {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub blueprint_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bundle_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub master_database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub master_password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub master_username: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub relational_database_name: terrars::PrimField<std::string::String>,
}

impl BuildLightsailDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> LightsailDatabase {
        let out = LightsailDatabase(std::rc::Rc::new(LightsailDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LightsailDatabaseData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                backup_retention_enabled: core::default::Default::default(),
                blueprint_id: self.blueprint_id,
                bundle_id: self.bundle_id,
                final_snapshot_name: core::default::Default::default(),
                id: core::default::Default::default(),
                master_database_name: self.master_database_name,
                master_password: self.master_password,
                master_username: self.master_username,
                preferred_backup_window: core::default::Default::default(),
                preferred_maintenance_window: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                relational_database_name: self.relational_database_name,
                skip_final_snapshot: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LightsailDatabaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_enabled` after provisioning.\n"]
    pub fn backup_retention_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blueprint_id` after provisioning.\n"]
    pub fn blueprint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blueprint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_identifier` after provisioning.\n"]
    pub fn ca_certificate_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_name` after provisioning.\n"]
    pub fn final_snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_database_name` after provisioning.\n"]
    pub fn master_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_address` after provisioning.\n"]
    pub fn master_endpoint_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_port` after provisioning.\n"]
    pub fn master_endpoint_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_size` after provisioning.\n"]
    pub fn ram_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ram_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_name` after provisioning.\n"]
    pub fn relational_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relational_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_availability_zone` after provisioning.\n"]
    pub fn secondary_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `support_code` after provisioning.\n"]
    pub fn support_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}
