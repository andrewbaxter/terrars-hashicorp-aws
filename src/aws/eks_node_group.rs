#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksNodeGroupData_ {
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
    ami_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    node_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_version: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<EksNodeGroup_LaunchTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_repair_config: std::option::Option<std::vec::Vec<EksNodeGroup_NodeRepairConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_access: std::option::Option<std::vec::Vec<EksNodeGroup_RemoteAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_config: std::option::Option<std::vec::Vec<EksNodeGroup_ScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<EksNodeGroup_Taint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EksNodeGroup_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_config: std::option::Option<std::vec::Vec<EksNodeGroup_UpdateConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_pool_config: std::option::Option<std::vec::Vec<EksNodeGroup_WarmPoolConfig>>,
    dynamic: EksNodeGroupDynamic,
}

struct EksNodeGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EksNodeGroupData_>,
}

#[derive(Clone)]
pub struct EksNodeGroup(std::rc::Rc<EksNodeGroup_>);

impl EksNodeGroup {
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

    #[doc = "Set the field `ami_type`.\n"]
    pub fn set_ami_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ami_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_type`.\n"]
    pub fn set_capacity_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().capacity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size`.\n"]
    pub fn set_disk_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_update_version`.\n"]
    pub fn set_force_update_version(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_types`.\n"]
    pub fn set_instance_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_group_name`.\n"]
    pub fn set_node_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_group_name_prefix`.\n"]
    pub fn set_node_group_name_prefix(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().node_group_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `release_version`.\n"]
    pub fn set_release_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().release_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_LaunchTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_repair_config`.\n"]
    pub fn set_node_repair_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_NodeRepairConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_repair_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_repair_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_access`.\n"]
    pub fn set_remote_access(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_RemoteAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_ScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(self, v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_Taint>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().taint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.taint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EksNodeGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_config`.\n"]
    pub fn set_update_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_UpdateConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warm_pool_config`.\n"]
    pub fn set_warm_pool_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroup_WarmPoolConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().warm_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.warm_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `ami_type` after provisioning.\n"]
    pub fn ami_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_type` after provisioning.\n"]
    pub fn capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name_prefix` after provisioning.\n"]
    pub fn node_group_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<EksNodeGroup_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<EksNodeGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_repair_config` after provisioning.\n"]
    pub fn node_repair_config(&self) -> terrars::ListRef<EksNodeGroup_NodeRepairConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<EksNodeGroup_RemoteAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<EksNodeGroup_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksNodeGroup_TimeoutsRef {
        EksNodeGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<EksNodeGroup_UpdateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool_config` after provisioning.\n"]
    pub fn warm_pool_config(&self) -> terrars::ListRef<EksNodeGroup_WarmPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool_config", self.extract_ref()))
    }
}

impl terrars::Referable for EksNodeGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EksNodeGroup { }

impl terrars::ToListMappable for EksNodeGroup {
    type O = terrars::ListRef<EksNodeGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EksNodeGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_eks_node_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksNodeGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub node_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksNodeGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> EksNodeGroup {
        let out = EksNodeGroup(std::rc::Rc::new(EksNodeGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EksNodeGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                ami_type: core::default::Default::default(),
                capacity_type: core::default::Default::default(),
                cluster_name: self.cluster_name,
                disk_size: core::default::Default::default(),
                force_update_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_types: core::default::Default::default(),
                labels: core::default::Default::default(),
                node_group_name: core::default::Default::default(),
                node_group_name_prefix: core::default::Default::default(),
                node_role_arn: self.node_role_arn,
                region: core::default::Default::default(),
                release_version: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                node_repair_config: core::default::Default::default(),
                remote_access: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                taint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_config: core::default::Default::default(),
                warm_pool_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksNodeGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EksNodeGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_type` after provisioning.\n"]
    pub fn ami_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_type` after provisioning.\n"]
    pub fn capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name_prefix` after provisioning.\n"]
    pub fn node_group_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<EksNodeGroup_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<EksNodeGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_repair_config` after provisioning.\n"]
    pub fn node_repair_config(&self) -> terrars::ListRef<EksNodeGroup_NodeRepairConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<EksNodeGroup_RemoteAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<EksNodeGroup_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksNodeGroup_TimeoutsRef {
        EksNodeGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<EksNodeGroup_UpdateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool_config` after provisioning.\n"]
    pub fn warm_pool_config(&self) -> terrars::ListRef<EksNodeGroup_WarmPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EksNodeGroupDynamic {
    launch_template: std::option::Option<terrars::DynamicBlock<EksNodeGroup_LaunchTemplate>>,
    node_repair_config: std::option::Option<terrars::DynamicBlock<EksNodeGroup_NodeRepairConfig>>,
    remote_access: std::option::Option<terrars::DynamicBlock<EksNodeGroup_RemoteAccess>>,
    scaling_config: std::option::Option<terrars::DynamicBlock<EksNodeGroup_ScalingConfig>>,
    taint: std::option::Option<terrars::DynamicBlock<EksNodeGroup_Taint>>,
    update_config: std::option::Option<terrars::DynamicBlock<EksNodeGroup_UpdateConfig>>,
    warm_pool_config: std::option::Option<terrars::DynamicBlock<EksNodeGroup_WarmPoolConfig>>,
}

#[derive(Serialize)]
pub struct EksNodeGroup_Resources_AutoscalingGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroup_Resources_AutoscalingGroups {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_Resources_AutoscalingGroups {
    type O = terrars::BlockAssignable<EksNodeGroup_Resources_AutoscalingGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_Resources_AutoscalingGroups {}

impl BuildEksNodeGroup_Resources_AutoscalingGroups {
    pub fn build(self) -> EksNodeGroup_Resources_AutoscalingGroups {
        EksNodeGroup_Resources_AutoscalingGroups { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_Resources_AutoscalingGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_Resources_AutoscalingGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_Resources_AutoscalingGroupsRef {
        EksNodeGroup_Resources_AutoscalingGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_Resources_AutoscalingGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_groups: std::option::Option<terrars::ListField<EksNodeGroup_Resources_AutoscalingGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_access_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroup_Resources {
    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<EksNodeGroup_Resources_AutoscalingGroups>>,
    ) -> Self {
        self.autoscaling_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remote_access_security_group_id`.\n"]
    pub fn set_remote_access_security_group_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.remote_access_security_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_Resources {
    type O = terrars::BlockAssignable<EksNodeGroup_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_Resources {}

impl BuildEksNodeGroup_Resources {
    pub fn build(self) -> EksNodeGroup_Resources {
        EksNodeGroup_Resources {
            autoscaling_groups: core::default::Default::default(),
            remote_access_security_group_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_ResourcesRef {
        EksNodeGroup_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> terrars::ListRef<EksNodeGroup_Resources_AutoscalingGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_access_security_group_id` after provisioning.\n"]
    pub fn remote_access_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_access_security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    version: terrars::PrimField<std::string::String>,
}

impl EksNodeGroup_LaunchTemplate {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_LaunchTemplate {
    type O = terrars::BlockAssignable<EksNodeGroup_LaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_LaunchTemplate {
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildEksNodeGroup_LaunchTemplate {
    pub fn build(self) -> EksNodeGroup_LaunchTemplate {
        EksNodeGroup_LaunchTemplate {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_LaunchTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_LaunchTemplateRef {
        EksNodeGroup_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_LaunchTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides {
    min_repair_wait_time_mins: terrars::PrimField<f64>,
    node_monitoring_condition: terrars::PrimField<std::string::String>,
    node_unhealthy_reason: terrars::PrimField<std::string::String>,
    repair_action: terrars::PrimField<std::string::String>,
}

impl EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides { }

impl terrars::ToListMappable for EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides {
    type O = terrars::BlockAssignable<EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides {
    #[doc = ""]
    pub min_repair_wait_time_mins: terrars::PrimField<f64>,
    #[doc = ""]
    pub node_monitoring_condition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub node_unhealthy_reason: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub repair_action: terrars::PrimField<std::string::String>,
}

impl BuildEksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides {
    pub fn build(self) -> EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides {
        EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides {
            min_repair_wait_time_mins: self.min_repair_wait_time_mins,
            node_monitoring_condition: self.node_monitoring_condition,
            node_unhealthy_reason: self.node_unhealthy_reason,
            repair_action: self.repair_action,
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverridesRef {
        EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `min_repair_wait_time_mins` after provisioning.\n"]
    pub fn min_repair_wait_time_mins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_repair_wait_time_mins", self.base))
    }

    #[doc = "Get a reference to the value of field `node_monitoring_condition` after provisioning.\n"]
    pub fn node_monitoring_condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_monitoring_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `node_unhealthy_reason` after provisioning.\n"]
    pub fn node_unhealthy_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_unhealthy_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `repair_action` after provisioning.\n"]
    pub fn repair_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repair_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksNodeGroup_NodeRepairConfigDynamic {
    node_repair_config_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides>>,
}

#[derive(Serialize)]
pub struct EksNodeGroup_NodeRepairConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_nodes_repaired_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_nodes_repaired_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unhealthy_node_threshold_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unhealthy_node_threshold_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_repair_config_overrides: std
    ::option
    ::Option<std::vec::Vec<EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides>>,
    dynamic: EksNodeGroup_NodeRepairConfigDynamic,
}

impl EksNodeGroup_NodeRepairConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_nodes_repaired_count`.\n"]
    pub fn set_max_parallel_nodes_repaired_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_parallel_nodes_repaired_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_nodes_repaired_percentage`.\n"]
    pub fn set_max_parallel_nodes_repaired_percentage(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_parallel_nodes_repaired_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_unhealthy_node_threshold_count`.\n"]
    pub fn set_max_unhealthy_node_threshold_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_unhealthy_node_threshold_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_unhealthy_node_threshold_percentage`.\n"]
    pub fn set_max_unhealthy_node_threshold_percentage(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_unhealthy_node_threshold_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_repair_config_overrides`.\n"]
    pub fn set_node_repair_config_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_repair_config_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_repair_config_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_NodeRepairConfig {
    type O = terrars::BlockAssignable<EksNodeGroup_NodeRepairConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_NodeRepairConfig {}

impl BuildEksNodeGroup_NodeRepairConfig {
    pub fn build(self) -> EksNodeGroup_NodeRepairConfig {
        EksNodeGroup_NodeRepairConfig {
            enabled: core::default::Default::default(),
            max_parallel_nodes_repaired_count: core::default::Default::default(),
            max_parallel_nodes_repaired_percentage: core::default::Default::default(),
            max_unhealthy_node_threshold_count: core::default::Default::default(),
            max_unhealthy_node_threshold_percentage: core::default::Default::default(),
            node_repair_config_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_NodeRepairConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_NodeRepairConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_NodeRepairConfigRef {
        EksNodeGroup_NodeRepairConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_NodeRepairConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_nodes_repaired_count` after provisioning.\n"]
    pub fn max_parallel_nodes_repaired_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_nodes_repaired_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_nodes_repaired_percentage` after provisioning.\n"]
    pub fn max_parallel_nodes_repaired_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_nodes_repaired_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_unhealthy_node_threshold_count` after provisioning.\n"]
    pub fn max_unhealthy_node_threshold_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unhealthy_node_threshold_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unhealthy_node_threshold_percentage` after provisioning.\n"]
    pub fn max_unhealthy_node_threshold_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unhealthy_node_threshold_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `node_repair_config_overrides` after provisioning.\n"]
    pub fn node_repair_config_overrides(
        &self,
    ) -> terrars::ListRef<EksNodeGroup_NodeRepairConfig_NodeRepairConfigOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_repair_config_overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_RemoteAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_ssh_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EksNodeGroup_RemoteAccess {
    #[doc = "Set the field `ec2_ssh_key`.\n"]
    pub fn set_ec2_ssh_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ec2_ssh_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_security_group_ids`.\n"]
    pub fn set_source_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_RemoteAccess {
    type O = terrars::BlockAssignable<EksNodeGroup_RemoteAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_RemoteAccess {}

impl BuildEksNodeGroup_RemoteAccess {
    pub fn build(self) -> EksNodeGroup_RemoteAccess {
        EksNodeGroup_RemoteAccess {
            ec2_ssh_key: core::default::Default::default(),
            source_security_group_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_RemoteAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_RemoteAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_RemoteAccessRef {
        EksNodeGroup_RemoteAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_RemoteAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ec2_ssh_key` after provisioning.\n"]
    pub fn ec2_ssh_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_ssh_key", self.base))
    }

    #[doc = "Get a reference to the value of field `source_security_group_ids` after provisioning.\n"]
    pub fn source_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_security_group_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_ScalingConfig {
    desired_size: terrars::PrimField<f64>,
    max_size: terrars::PrimField<f64>,
    min_size: terrars::PrimField<f64>,
}

impl EksNodeGroup_ScalingConfig { }

impl terrars::ToListMappable for EksNodeGroup_ScalingConfig {
    type O = terrars::BlockAssignable<EksNodeGroup_ScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_ScalingConfig {
    #[doc = ""]
    pub desired_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_size: terrars::PrimField<f64>,
}

impl BuildEksNodeGroup_ScalingConfig {
    pub fn build(self) -> EksNodeGroup_ScalingConfig {
        EksNodeGroup_ScalingConfig {
            desired_size: self.desired_size,
            max_size: self.max_size,
            min_size: self.min_size,
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_ScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_ScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_ScalingConfigRef {
        EksNodeGroup_ScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_ScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_size` after provisioning.\n"]
    pub fn desired_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_size", self.base))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.base))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_Taint {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroup_Taint {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_Taint {
    type O = terrars::BlockAssignable<EksNodeGroup_Taint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_Taint {
    #[doc = ""]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildEksNodeGroup_Taint {
    pub fn build(self) -> EksNodeGroup_Taint {
        EksNodeGroup_Taint {
            effect: self.effect,
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_TaintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_TaintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_TaintRef {
        EksNodeGroup_TaintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_TaintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroup_Timeouts {
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

impl terrars::ToListMappable for EksNodeGroup_Timeouts {
    type O = terrars::BlockAssignable<EksNodeGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_Timeouts {}

impl BuildEksNodeGroup_Timeouts {
    pub fn build(self) -> EksNodeGroup_Timeouts {
        EksNodeGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_TimeoutsRef {
        EksNodeGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_TimeoutsRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct EksNodeGroup_UpdateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroup_UpdateConfig {
    #[doc = "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_unavailable_percentage`.\n"]
    pub fn set_max_unavailable_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_strategy`.\n"]
    pub fn set_update_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_UpdateConfig {
    type O = terrars::BlockAssignable<EksNodeGroup_UpdateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_UpdateConfig {}

impl BuildEksNodeGroup_UpdateConfig {
    pub fn build(self) -> EksNodeGroup_UpdateConfig {
        EksNodeGroup_UpdateConfig {
            max_unavailable: core::default::Default::default(),
            max_unavailable_percentage: core::default::Default::default(),
            update_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_UpdateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_UpdateConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_UpdateConfigRef {
        EksNodeGroup_UpdateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_UpdateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unavailable_percentage` after provisioning.\n"]
    pub fn max_unavailable_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `update_strategy` after provisioning.\n"]
    pub fn update_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroup_WarmPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_group_prepared_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_on_scale_in: std::option::Option<terrars::PrimField<bool>>,
}

impl EksNodeGroup_WarmPoolConfig {
    #[doc = "Set the field `max_group_prepared_capacity`.\n"]
    pub fn set_max_group_prepared_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_group_prepared_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_size`.\n"]
    pub fn set_min_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_state`.\n"]
    pub fn set_pool_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pool_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reuse_on_scale_in`.\n"]
    pub fn set_reuse_on_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reuse_on_scale_in = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroup_WarmPoolConfig {
    type O = terrars::BlockAssignable<EksNodeGroup_WarmPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroup_WarmPoolConfig {}

impl BuildEksNodeGroup_WarmPoolConfig {
    pub fn build(self) -> EksNodeGroup_WarmPoolConfig {
        EksNodeGroup_WarmPoolConfig {
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
            reuse_on_scale_in: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksNodeGroup_WarmPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroup_WarmPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroup_WarmPoolConfigRef {
        EksNodeGroup_WarmPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroup_WarmPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_group_prepared_capacity` after provisioning.\n"]
    pub fn max_group_prepared_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_group_prepared_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_state` after provisioning.\n"]
    pub fn pool_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_state", self.base))
    }

    #[doc = "Get a reference to the value of field `reuse_on_scale_in` after provisioning.\n"]
    pub fn reuse_on_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reuse_on_scale_in", self.base))
    }
}
