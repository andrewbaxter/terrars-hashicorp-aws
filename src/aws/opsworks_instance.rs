use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_ebs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_eip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_optimized: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecs_cluster_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elastic_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infrastructure_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_updates_on_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_profile_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    layer_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_device_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_key_name: Option<PrimField<String>>,
    stack_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenancy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtualization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_block_device: Option<Vec<OpsworksInstanceEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_block_device: Option<Vec<OpsworksInstanceEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_block_device: Option<Vec<OpsworksInstanceRootBlockDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OpsworksInstanceTimeoutsEl>,
    dynamic: OpsworksInstanceDynamic,
}

struct OpsworksInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpsworksInstanceData>,
}

#[derive(Clone)]
pub struct OpsworksInstance(Rc<OpsworksInstance_>);

impl OpsworksInstance {
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

    #[doc= "Set the field `agent_version`.\n"]
    pub fn set_agent_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().agent_version = Some(v.into());
        self
    }

    #[doc= "Set the field `ami_id`.\n"]
    pub fn set_ami_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ami_id = Some(v.into());
        self
    }

    #[doc= "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().architecture = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_scaling_type`.\n"]
    pub fn set_auto_scaling_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_scaling_type = Some(v.into());
        self
    }

    #[doc= "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_ebs`.\n"]
    pub fn set_delete_ebs(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_ebs = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_eip`.\n"]
    pub fn set_delete_eip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_eip = Some(v.into());
        self
    }

    #[doc= "Set the field `ebs_optimized`.\n"]
    pub fn set_ebs_optimized(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ebs_optimized = Some(v.into());
        self
    }

    #[doc= "Set the field `ecs_cluster_arn`.\n"]
    pub fn set_ecs_cluster_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ecs_cluster_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `elastic_ip`.\n"]
    pub fn set_elastic_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elastic_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `infrastructure_class`.\n"]
    pub fn set_infrastructure_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().infrastructure_class = Some(v.into());
        self
    }

    #[doc= "Set the field `install_updates_on_boot`.\n"]
    pub fn set_install_updates_on_boot(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().install_updates_on_boot = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_profile_arn`.\n"]
    pub fn set_instance_profile_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_profile_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `os`.\n"]
    pub fn set_os(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().os = Some(v.into());
        self
    }

    #[doc= "Set the field `root_device_type`.\n"]
    pub fn set_root_device_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `ssh_key_name`.\n"]
    pub fn set_ssh_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssh_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = Some(v.into());
        self
    }

    #[doc= "Set the field `tenancy`.\n"]
    pub fn set_tenancy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tenancy = Some(v.into());
        self
    }

    #[doc= "Set the field `virtualization_type`.\n"]
    pub fn set_virtualization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().virtualization_type = Some(v.into());
        self
    }

    #[doc= "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(self, v: impl Into<BlockAssignable<OpsworksInstanceEbsBlockDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_block_device = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        self,
        v: impl Into<BlockAssignable<OpsworksInstanceEphemeralBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_block_device = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(self, v: impl Into<BlockAssignable<OpsworksInstanceRootBlockDeviceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_block_device = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_block_device = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OpsworksInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ami_id` after provisioning.\n"]
    pub fn ami_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_scaling_type` after provisioning.\n"]
    pub fn auto_scaling_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_ebs` after provisioning.\n"]
    pub fn delete_ebs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_ebs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_eip` after provisioning.\n"]
    pub fn delete_eip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_eip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_class` after provisioning.\n"]
    pub fn infrastructure_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_profile_arn` after provisioning.\n"]
    pub fn instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_service_error_id` after provisioning.\n"]
    pub fn last_service_error_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_service_error_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `layer_ids` after provisioning.\n"]
    pub fn layer_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.layer_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `os` after provisioning.\n"]
    pub fn os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registered_by` after provisioning.\n"]
    pub fn registered_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registered_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_agent_version` after provisioning.\n"]
    pub fn reported_agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_os_family` after provisioning.\n"]
    pub fn reported_os_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_os_family", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_os_name` after provisioning.\n"]
    pub fn reported_os_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_os_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_os_version` after provisioning.\n"]
    pub fn reported_os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_os_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_device_type` after provisioning.\n"]
    pub fn root_device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_device_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_device_volume_id` after provisioning.\n"]
    pub fn root_device_volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_device_volume_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_host_dsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_dsa_key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_dsa_key_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_host_rsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_rsa_key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_rsa_key_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_key_name` after provisioning.\n"]
    pub fn ssh_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `virtualization_type` after provisioning.\n"]
    pub fn virtualization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtualization_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksInstanceTimeoutsElRef {
        OpsworksInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for OpsworksInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for OpsworksInstance {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for OpsworksInstance {
    type O = ListRef<OpsworksInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for OpsworksInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_opsworks_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksInstance {
    pub tf_id: String,
    #[doc= ""]
    pub layer_ids: ListField<PrimField<String>>,
    #[doc= ""]
    pub stack_id: PrimField<String>,
}

impl BuildOpsworksInstance {
    pub fn build(self, stack: &mut Stack) -> OpsworksInstance {
        let out = OpsworksInstance(Rc::new(OpsworksInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpsworksInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_version: core::default::Default::default(),
                ami_id: core::default::Default::default(),
                architecture: core::default::Default::default(),
                auto_scaling_type: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                created_at: core::default::Default::default(),
                delete_ebs: core::default::Default::default(),
                delete_eip: core::default::Default::default(),
                ebs_optimized: core::default::Default::default(),
                ecs_cluster_arn: core::default::Default::default(),
                elastic_ip: core::default::Default::default(),
                hostname: core::default::Default::default(),
                id: core::default::Default::default(),
                infrastructure_class: core::default::Default::default(),
                install_updates_on_boot: core::default::Default::default(),
                instance_profile_arn: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                layer_ids: self.layer_ids,
                os: core::default::Default::default(),
                root_device_type: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                ssh_key_name: core::default::Default::default(),
                stack_id: self.stack_id,
                state: core::default::Default::default(),
                status: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tenancy: core::default::Default::default(),
                virtualization_type: core::default::Default::default(),
                ebs_block_device: core::default::Default::default(),
                ephemeral_block_device: core::default::Default::default(),
                root_block_device: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ami_id` after provisioning.\n"]
    pub fn ami_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_scaling_type` after provisioning.\n"]
    pub fn auto_scaling_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_ebs` after provisioning.\n"]
    pub fn delete_ebs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_ebs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_eip` after provisioning.\n"]
    pub fn delete_eip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_eip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_class` after provisioning.\n"]
    pub fn infrastructure_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_profile_arn` after provisioning.\n"]
    pub fn instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_service_error_id` after provisioning.\n"]
    pub fn last_service_error_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_service_error_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `layer_ids` after provisioning.\n"]
    pub fn layer_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.layer_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `os` after provisioning.\n"]
    pub fn os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registered_by` after provisioning.\n"]
    pub fn registered_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registered_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_agent_version` after provisioning.\n"]
    pub fn reported_agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_os_family` after provisioning.\n"]
    pub fn reported_os_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_os_family", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_os_name` after provisioning.\n"]
    pub fn reported_os_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_os_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reported_os_version` after provisioning.\n"]
    pub fn reported_os_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reported_os_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_device_type` after provisioning.\n"]
    pub fn root_device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_device_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_device_volume_id` after provisioning.\n"]
    pub fn root_device_volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_device_volume_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_host_dsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_dsa_key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_dsa_key_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_host_rsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_rsa_key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_rsa_key_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_key_name` after provisioning.\n"]
    pub fn ssh_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `virtualization_type` after provisioning.\n"]
    pub fn virtualization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtualization_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksInstanceTimeoutsElRef {
        OpsworksInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl OpsworksInstanceEbsBlockDeviceEl {
    #[doc= "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc= "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc= "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_id = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksInstanceEbsBlockDeviceEl {
    type O = BlockAssignable<OpsworksInstanceEbsBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceEbsBlockDeviceEl {
    #[doc= ""]
    pub device_name: PrimField<String>,
}

impl BuildOpsworksInstanceEbsBlockDeviceEl {
    pub fn build(self) -> OpsworksInstanceEbsBlockDeviceEl {
        OpsworksInstanceEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            iops: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct OpsworksInstanceEbsBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksInstanceEbsBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> OpsworksInstanceEbsBlockDeviceElRef {
        OpsworksInstanceEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceEbsBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc= "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceEphemeralBlockDeviceEl {
    device_name: PrimField<String>,
    virtual_name: PrimField<String>,
}

impl OpsworksInstanceEphemeralBlockDeviceEl { }

impl ToListMappable for OpsworksInstanceEphemeralBlockDeviceEl {
    type O = BlockAssignable<OpsworksInstanceEphemeralBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceEphemeralBlockDeviceEl {
    #[doc= ""]
    pub device_name: PrimField<String>,
    #[doc= ""]
    pub virtual_name: PrimField<String>,
}

impl BuildOpsworksInstanceEphemeralBlockDeviceEl {
    pub fn build(self) -> OpsworksInstanceEphemeralBlockDeviceEl {
        OpsworksInstanceEphemeralBlockDeviceEl {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
        }
    }
}

pub struct OpsworksInstanceEphemeralBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksInstanceEphemeralBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> OpsworksInstanceEphemeralBlockDeviceElRef {
        OpsworksInstanceEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceEphemeralBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceRootBlockDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl OpsworksInstanceRootBlockDeviceEl {
    #[doc= "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.delete_on_termination = Some(v.into());
        self
    }

    #[doc= "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksInstanceRootBlockDeviceEl {
    type O = BlockAssignable<OpsworksInstanceRootBlockDeviceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceRootBlockDeviceEl {}

impl BuildOpsworksInstanceRootBlockDeviceEl {
    pub fn build(self) -> OpsworksInstanceRootBlockDeviceEl {
        OpsworksInstanceRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            iops: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct OpsworksInstanceRootBlockDeviceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksInstanceRootBlockDeviceElRef {
    fn new(shared: StackShared, base: String) -> OpsworksInstanceRootBlockDeviceElRef {
        OpsworksInstanceRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceRootBlockDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc= "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl OpsworksInstanceTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksInstanceTimeoutsEl {
    type O = BlockAssignable<OpsworksInstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceTimeoutsEl {}

impl BuildOpsworksInstanceTimeoutsEl {
    pub fn build(self) -> OpsworksInstanceTimeoutsEl {
        OpsworksInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OpsworksInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OpsworksInstanceTimeoutsElRef {
        OpsworksInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksInstanceDynamic {
    ebs_block_device: Option<DynamicBlock<OpsworksInstanceEbsBlockDeviceEl>>,
    ephemeral_block_device: Option<DynamicBlock<OpsworksInstanceEphemeralBlockDeviceEl>>,
    root_block_device: Option<DynamicBlock<OpsworksInstanceRootBlockDeviceEl>>,
}
