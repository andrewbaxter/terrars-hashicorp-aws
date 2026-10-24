use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftClusterData {
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
    allow_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apply_immediately: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aqua_configuration_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_snapshot_retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_relocation_enabled: std::option::Option<terrars::PrimField<bool>>,
    cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_parameter_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_public_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_revision_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_subnet_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_iam_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_vpc_routing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    final_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_track_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_snapshot_retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_username: std::option::Option<terrars::PrimField<std::string::String>>,
    node_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_maintenance_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_final_snapshot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_cluster_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<RedshiftClusterLoggingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_copy: std::option::Option<std::vec::Vec<RedshiftClusterSnapshotCopyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RedshiftClusterTimeoutsEl>,
    dynamic: RedshiftClusterDynamic,
}

struct RedshiftCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftClusterData>,
}

#[derive(Clone)]
pub struct RedshiftCluster(std::rc::Rc<RedshiftCluster_>);

impl RedshiftCluster {
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

    #[doc = "Set the field `allow_version_upgrade`.\n"]
    pub fn set_allow_version_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aqua_configuration_status`.\n"]
    pub fn set_aqua_configuration_status(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().aqua_configuration_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automated_snapshot_retention_period`.\n"]
    pub fn set_automated_snapshot_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().automated_snapshot_retention_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone_relocation_enabled`.\n"]
    pub fn set_availability_zone_relocation_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().availability_zone_relocation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_parameter_group_name`.\n"]
    pub fn set_cluster_parameter_group_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_parameter_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_public_key`.\n"]
    pub fn set_cluster_public_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_revision_number`.\n"]
    pub fn set_cluster_revision_number(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_revision_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_security_groups`.\n"]
    pub fn set_cluster_security_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_subnet_group_name`.\n"]
    pub fn set_cluster_subnet_group_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_subnet_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_version`.\n"]
    pub fn set_cluster_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_iam_role_arn`.\n"]
    pub fn set_default_iam_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_iam_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_ip`.\n"]
    pub fn set_elastic_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().elastic_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enhanced_vpc_routing`.\n"]
    pub fn set_enhanced_vpc_routing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_vpc_routing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `final_snapshot_identifier`.\n"]
    pub fn set_final_snapshot_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().final_snapshot_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_roles`.\n"]
    pub fn set_iam_roles(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().iam_roles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_track_name`.\n"]
    pub fn set_maintenance_track_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().maintenance_track_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manual_snapshot_retention_period`.\n"]
    pub fn set_manual_snapshot_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().manual_snapshot_retention_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_password`.\n"]
    pub fn set_master_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().master_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_username`.\n"]
    pub fn set_master_username(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().master_username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_nodes`.\n"]
    pub fn set_number_of_nodes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_account`.\n"]
    pub fn set_owner_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().owner_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `snapshot_cluster_identifier`.\n"]
    pub fn set_snapshot_cluster_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().snapshot_cluster_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_identifier`.\n"]
    pub fn set_snapshot_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().snapshot_identifier = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(self, v: impl std::convert::Into<terrars::BlockAssignable<RedshiftClusterLoggingEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_copy`.\n"]
    pub fn set_snapshot_copy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftClusterSnapshotCopyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_copy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_copy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RedshiftClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_version_upgrade` after provisioning.\n"]
    pub fn allow_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aqua_configuration_status` after provisioning.\n"]
    pub fn aqua_configuration_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aqua_configuration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_retention_period` after provisioning.\n"]
    pub fn automated_snapshot_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.automated_snapshot_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_relocation_enabled` after provisioning.\n"]
    pub fn availability_zone_relocation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_relocation_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_nodes` after provisioning.\n"]
    pub fn cluster_nodes(&self) -> terrars::ListRef<RedshiftClusterClusterNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_parameter_group_name` after provisioning.\n"]
    pub fn cluster_parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_public_key` after provisioning.\n"]
    pub fn cluster_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_revision_number` after provisioning.\n"]
    pub fn cluster_revision_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_revision_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_security_groups` after provisioning.\n"]
    pub fn cluster_security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cluster_security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet_group_name` after provisioning.\n"]
    pub fn cluster_subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_version` after provisioning.\n"]
    pub fn cluster_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_iam_role_arn` after provisioning.\n"]
    pub fn default_iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_vpc_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.iam_roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_track_name` after provisioning.\n"]
    pub fn maintenance_track_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_track_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_snapshot_retention_period` after provisioning.\n"]
    pub fn manual_snapshot_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_snapshot_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account` after provisioning.\n"]
    pub fn owner_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_cluster_identifier` after provisioning.\n"]
    pub fn snapshot_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<RedshiftClusterLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_copy` after provisioning.\n"]
    pub fn snapshot_copy(&self) -> terrars::ListRef<RedshiftClusterSnapshotCopyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_copy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftClusterTimeoutsElRef {
        RedshiftClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for RedshiftCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedshiftCluster { }

impl terrars::ToListMappable for RedshiftCluster {
    type O = terrars::ListRef<RedshiftClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedshiftCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_redshift_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub node_type: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> RedshiftCluster {
        let out = RedshiftCluster(std::rc::Rc::new(RedshiftCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedshiftClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_version_upgrade: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                aqua_configuration_status: core::default::Default::default(),
                automated_snapshot_retention_period: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                availability_zone_relocation_enabled: core::default::Default::default(),
                cluster_identifier: self.cluster_identifier,
                cluster_parameter_group_name: core::default::Default::default(),
                cluster_public_key: core::default::Default::default(),
                cluster_revision_number: core::default::Default::default(),
                cluster_security_groups: core::default::Default::default(),
                cluster_subnet_group_name: core::default::Default::default(),
                cluster_type: core::default::Default::default(),
                cluster_version: core::default::Default::default(),
                database_name: core::default::Default::default(),
                default_iam_role_arn: core::default::Default::default(),
                elastic_ip: core::default::Default::default(),
                encrypted: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                enhanced_vpc_routing: core::default::Default::default(),
                final_snapshot_identifier: core::default::Default::default(),
                iam_roles: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                maintenance_track_name: core::default::Default::default(),
                manual_snapshot_retention_period: core::default::Default::default(),
                master_password: core::default::Default::default(),
                master_username: core::default::Default::default(),
                node_type: self.node_type,
                number_of_nodes: core::default::Default::default(),
                owner_account: core::default::Default::default(),
                port: core::default::Default::default(),
                preferred_maintenance_window: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                skip_final_snapshot: core::default::Default::default(),
                snapshot_cluster_identifier: core::default::Default::default(),
                snapshot_identifier: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                logging: core::default::Default::default(),
                snapshot_copy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_version_upgrade` after provisioning.\n"]
    pub fn allow_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aqua_configuration_status` after provisioning.\n"]
    pub fn aqua_configuration_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aqua_configuration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_retention_period` after provisioning.\n"]
    pub fn automated_snapshot_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.automated_snapshot_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_relocation_enabled` after provisioning.\n"]
    pub fn availability_zone_relocation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_relocation_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_nodes` after provisioning.\n"]
    pub fn cluster_nodes(&self) -> terrars::ListRef<RedshiftClusterClusterNodesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_parameter_group_name` after provisioning.\n"]
    pub fn cluster_parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_public_key` after provisioning.\n"]
    pub fn cluster_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_revision_number` after provisioning.\n"]
    pub fn cluster_revision_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_revision_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_security_groups` after provisioning.\n"]
    pub fn cluster_security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cluster_security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet_group_name` after provisioning.\n"]
    pub fn cluster_subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_version` after provisioning.\n"]
    pub fn cluster_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_iam_role_arn` after provisioning.\n"]
    pub fn default_iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_vpc_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_identifier` after provisioning.\n"]
    pub fn final_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.iam_roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_track_name` after provisioning.\n"]
    pub fn maintenance_track_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_track_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_snapshot_retention_period` after provisioning.\n"]
    pub fn manual_snapshot_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_snapshot_retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account` after provisioning.\n"]
    pub fn owner_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_cluster_identifier` after provisioning.\n"]
    pub fn snapshot_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_identifier` after provisioning.\n"]
    pub fn snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<RedshiftClusterLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_copy` after provisioning.\n"]
    pub fn snapshot_copy(&self) -> terrars::ListRef<RedshiftClusterSnapshotCopyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_copy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftClusterTimeoutsElRef {
        RedshiftClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftClusterClusterNodesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftClusterClusterNodesEl {
    #[doc = "Set the field `node_role`.\n"]
    pub fn set_node_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip_address`.\n"]
    pub fn set_public_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_ip_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftClusterClusterNodesEl {
    type O = terrars::BlockAssignable<RedshiftClusterClusterNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftClusterClusterNodesEl {}

impl BuildRedshiftClusterClusterNodesEl {
    pub fn build(self) -> RedshiftClusterClusterNodesEl {
        RedshiftClusterClusterNodesEl {
            node_role: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            public_ip_address: core::default::Default::default(),
        }
    }
}

pub struct RedshiftClusterClusterNodesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftClusterClusterNodesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftClusterClusterNodesElRef {
        RedshiftClusterClusterNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftClusterClusterNodesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_role` after provisioning.\n"]
    pub fn node_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_role", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\n"]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftClusterLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    enable: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_exports: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftClusterLoggingEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination_type`.\n"]
    pub fn set_log_destination_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_destination_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_exports`.\n"]
    pub fn set_log_exports(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.log_exports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftClusterLoggingEl {
    type O = terrars::BlockAssignable<RedshiftClusterLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftClusterLoggingEl {
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
}

impl BuildRedshiftClusterLoggingEl {
    pub fn build(self) -> RedshiftClusterLoggingEl {
        RedshiftClusterLoggingEl {
            bucket_name: core::default::Default::default(),
            enable: self.enable,
            log_destination_type: core::default::Default::default(),
            log_exports: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct RedshiftClusterLoggingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftClusterLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftClusterLoggingElRef {
        RedshiftClusterLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftClusterLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_exports` after provisioning.\n"]
    pub fn log_exports(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_exports", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftClusterSnapshotCopyEl {
    destination_region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<f64>>,
}

impl RedshiftClusterSnapshotCopyEl {
    #[doc = "Set the field `grant_name`.\n"]
    pub fn set_grant_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grant_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftClusterSnapshotCopyEl {
    type O = terrars::BlockAssignable<RedshiftClusterSnapshotCopyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftClusterSnapshotCopyEl {
    #[doc = ""]
    pub destination_region: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftClusterSnapshotCopyEl {
    pub fn build(self) -> RedshiftClusterSnapshotCopyEl {
        RedshiftClusterSnapshotCopyEl {
            destination_region: self.destination_region,
            grant_name: core::default::Default::default(),
            retention_period: core::default::Default::default(),
        }
    }
}

pub struct RedshiftClusterSnapshotCopyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftClusterSnapshotCopyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftClusterSnapshotCopyElRef {
        RedshiftClusterSnapshotCopyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftClusterSnapshotCopyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_region` after provisioning.\n"]
    pub fn destination_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_region", self.base))
    }

    #[doc = "Get a reference to the value of field `grant_name` after provisioning.\n"]
    pub fn grant_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_name", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftClusterTimeoutsEl {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftClusterTimeoutsEl {
    type O = terrars::BlockAssignable<RedshiftClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftClusterTimeoutsEl {}

impl BuildRedshiftClusterTimeoutsEl {
    pub fn build(self) -> RedshiftClusterTimeoutsEl {
        RedshiftClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RedshiftClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftClusterTimeoutsElRef {
        RedshiftClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftClusterTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftClusterDynamic {
    logging: std::option::Option<terrars::DynamicBlock<RedshiftClusterLoggingEl>>,
    snapshot_copy: std::option::Option<terrars::DynamicBlock<RedshiftClusterSnapshotCopyEl>>,
}
