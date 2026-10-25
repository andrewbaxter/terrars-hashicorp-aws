#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DrsReplicationConfigurationTemplateData_ {
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
    associate_default_security_group: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_replicate_new_disks: std::option::Option<terrars::PrimField<bool>>,
    bandwidth_throttling: terrars::PrimField<f64>,
    create_public_ip: terrars::PrimField<bool>,
    data_plane_routing: terrars::PrimField<std::string::String>,
    default_large_staging_disk_type: terrars::PrimField<std::string::String>,
    ebs_encryption: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    replication_server_instance_type: terrars::PrimField<std::string::String>,
    replication_servers_security_groups_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    staging_area_subnet_id: terrars::PrimField<std::string::String>,
    staging_area_tags: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    use_dedicated_replication_server: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pit_policy: std::option::Option<std::vec::Vec<DrsReplicationConfigurationTemplate_PitPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DrsReplicationConfigurationTemplate_Timeouts>,
    dynamic: DrsReplicationConfigurationTemplateDynamic,
}

struct DrsReplicationConfigurationTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DrsReplicationConfigurationTemplateData_>,
}

#[derive(Clone)]
pub struct DrsReplicationConfigurationTemplate(std::rc::Rc<DrsReplicationConfigurationTemplate_>);

impl DrsReplicationConfigurationTemplate {
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

    #[doc = "Set the field `auto_replicate_new_disks`.\n"]
    pub fn set_auto_replicate_new_disks(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_replicate_new_disks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_encryption_key_arn`.\n"]
    pub fn set_ebs_encryption_key_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ebs_encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `pit_policy`.\n"]
    pub fn set_pit_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DrsReplicationConfigurationTemplate_PitPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pit_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pit_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DrsReplicationConfigurationTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_default_security_group` after provisioning.\n"]
    pub fn associate_default_security_group(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_default_security_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_replicate_new_disks` after provisioning.\n"]
    pub fn auto_replicate_new_disks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_replicate_new_disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth_throttling` after provisioning.\n"]
    pub fn bandwidth_throttling(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_throttling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_public_ip` after provisioning.\n"]
    pub fn create_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_plane_routing` after provisioning.\n"]
    pub fn data_plane_routing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_plane_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_large_staging_disk_type` after provisioning.\n"]
    pub fn default_large_staging_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_large_staging_disk_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_encryption` after provisioning.\n"]
    pub fn ebs_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption_key_arn` after provisioning.\n"]
    pub fn ebs_encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_server_instance_type` after provisioning.\n"]
    pub fn replication_server_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_server_instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_servers_security_groups_ids` after provisioning.\n"]
    pub fn replication_servers_security_groups_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.replication_servers_security_groups_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `staging_area_subnet_id` after provisioning.\n"]
    pub fn staging_area_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_area_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging_area_tags` after provisioning.\n"]
    pub fn staging_area_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.staging_area_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_dedicated_replication_server` after provisioning.\n"]
    pub fn use_dedicated_replication_server(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_dedicated_replication_server", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pit_policy` after provisioning.\n"]
    pub fn pit_policy(&self) -> terrars::ListRef<DrsReplicationConfigurationTemplate_PitPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pit_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DrsReplicationConfigurationTemplate_TimeoutsRef {
        DrsReplicationConfigurationTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DrsReplicationConfigurationTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DrsReplicationConfigurationTemplate { }

impl terrars::ToListMappable for DrsReplicationConfigurationTemplate {
    type O = terrars::ListRef<DrsReplicationConfigurationTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DrsReplicationConfigurationTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_drs_replication_configuration_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDrsReplicationConfigurationTemplate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub associate_default_security_group: terrars::PrimField<bool>,
    #[doc = ""]
    pub bandwidth_throttling: terrars::PrimField<f64>,
    #[doc = ""]
    pub create_public_ip: terrars::PrimField<bool>,
    #[doc = ""]
    pub data_plane_routing: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub default_large_staging_disk_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ebs_encryption: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub replication_server_instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub replication_servers_security_groups_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub staging_area_subnet_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub staging_area_tags: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub use_dedicated_replication_server: terrars::PrimField<bool>,
}

impl BuildDrsReplicationConfigurationTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DrsReplicationConfigurationTemplate {
        let out = DrsReplicationConfigurationTemplate(std::rc::Rc::new(DrsReplicationConfigurationTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DrsReplicationConfigurationTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                associate_default_security_group: self.associate_default_security_group,
                auto_replicate_new_disks: core::default::Default::default(),
                bandwidth_throttling: self.bandwidth_throttling,
                create_public_ip: self.create_public_ip,
                data_plane_routing: self.data_plane_routing,
                default_large_staging_disk_type: self.default_large_staging_disk_type,
                ebs_encryption: self.ebs_encryption,
                ebs_encryption_key_arn: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_server_instance_type: self.replication_server_instance_type,
                replication_servers_security_groups_ids: self.replication_servers_security_groups_ids,
                staging_area_subnet_id: self.staging_area_subnet_id,
                staging_area_tags: self.staging_area_tags,
                tags: core::default::Default::default(),
                use_dedicated_replication_server: self.use_dedicated_replication_server,
                pit_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DrsReplicationConfigurationTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DrsReplicationConfigurationTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DrsReplicationConfigurationTemplateRef {
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

    #[doc = "Get a reference to the value of field `associate_default_security_group` after provisioning.\n"]
    pub fn associate_default_security_group(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.associate_default_security_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_replicate_new_disks` after provisioning.\n"]
    pub fn auto_replicate_new_disks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_replicate_new_disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth_throttling` after provisioning.\n"]
    pub fn bandwidth_throttling(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_throttling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_public_ip` after provisioning.\n"]
    pub fn create_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_plane_routing` after provisioning.\n"]
    pub fn data_plane_routing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_plane_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_large_staging_disk_type` after provisioning.\n"]
    pub fn default_large_staging_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_large_staging_disk_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_encryption` after provisioning.\n"]
    pub fn ebs_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption_key_arn` after provisioning.\n"]
    pub fn ebs_encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_server_instance_type` after provisioning.\n"]
    pub fn replication_server_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_server_instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_servers_security_groups_ids` after provisioning.\n"]
    pub fn replication_servers_security_groups_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.replication_servers_security_groups_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `staging_area_subnet_id` after provisioning.\n"]
    pub fn staging_area_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_area_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging_area_tags` after provisioning.\n"]
    pub fn staging_area_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.staging_area_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_dedicated_replication_server` after provisioning.\n"]
    pub fn use_dedicated_replication_server(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_dedicated_replication_server", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pit_policy` after provisioning.\n"]
    pub fn pit_policy(&self) -> terrars::ListRef<DrsReplicationConfigurationTemplate_PitPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pit_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DrsReplicationConfigurationTemplate_TimeoutsRef {
        DrsReplicationConfigurationTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DrsReplicationConfigurationTemplateDynamic {
    pit_policy: std::option::Option<terrars::DynamicBlock<DrsReplicationConfigurationTemplate_PitPolicy>>,
}

#[derive(Serialize)]
pub struct DrsReplicationConfigurationTemplate_PitPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    interval: terrars::PrimField<f64>,
    retention_duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<f64>>,
    units: terrars::PrimField<std::string::String>,
}

impl DrsReplicationConfigurationTemplate_PitPolicy {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_id`.\n"]
    pub fn set_rule_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rule_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DrsReplicationConfigurationTemplate_PitPolicy {
    type O = terrars::BlockAssignable<DrsReplicationConfigurationTemplate_PitPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDrsReplicationConfigurationTemplate_PitPolicy {
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub retention_duration: terrars::PrimField<f64>,
    #[doc = ""]
    pub units: terrars::PrimField<std::string::String>,
}

impl BuildDrsReplicationConfigurationTemplate_PitPolicy {
    pub fn build(self) -> DrsReplicationConfigurationTemplate_PitPolicy {
        DrsReplicationConfigurationTemplate_PitPolicy {
            enabled: core::default::Default::default(),
            interval: self.interval,
            retention_duration: self.retention_duration,
            rule_id: core::default::Default::default(),
            units: self.units,
        }
    }
}

#[allow(dead_code)]
pub struct DrsReplicationConfigurationTemplate_PitPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DrsReplicationConfigurationTemplate_PitPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DrsReplicationConfigurationTemplate_PitPolicyRef {
        DrsReplicationConfigurationTemplate_PitPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DrsReplicationConfigurationTemplate_PitPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_duration` after provisioning.\n"]
    pub fn retention_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct DrsReplicationConfigurationTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DrsReplicationConfigurationTemplate_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DrsReplicationConfigurationTemplate_Timeouts {
    type O = terrars::BlockAssignable<DrsReplicationConfigurationTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDrsReplicationConfigurationTemplate_Timeouts {}

impl BuildDrsReplicationConfigurationTemplate_Timeouts {
    pub fn build(self) -> DrsReplicationConfigurationTemplate_Timeouts {
        DrsReplicationConfigurationTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DrsReplicationConfigurationTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DrsReplicationConfigurationTemplate_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DrsReplicationConfigurationTemplate_TimeoutsRef {
        DrsReplicationConfigurationTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DrsReplicationConfigurationTemplate_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
