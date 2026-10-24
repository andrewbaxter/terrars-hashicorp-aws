use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksNodeGroupData {
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
    release_version: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<EksNodeGroupLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_access: std::option::Option<std::vec::Vec<EksNodeGroupRemoteAccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_config: std::option::Option<std::vec::Vec<EksNodeGroupScalingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<EksNodeGroupTaintEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EksNodeGroupTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_config: std::option::Option<std::vec::Vec<EksNodeGroupUpdateConfigEl>>,
    dynamic: EksNodeGroupDynamic,
}

struct EksNodeGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EksNodeGroupData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroupLaunchTemplateEl>>,
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

    #[doc = "Set the field `remote_access`.\n"]
    pub fn set_remote_access(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroupRemoteAccessEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroupScalingConfigEl>>,
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
    pub fn set_taint(self, v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroupTaintEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<EksNodeGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_config`.\n"]
    pub fn set_update_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksNodeGroupUpdateConfigEl>>,
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

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<EksNodeGroupResourcesElRef> {
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
    pub fn launch_template(&self) -> terrars::ListRef<EksNodeGroupLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<EksNodeGroupRemoteAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<EksNodeGroupScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksNodeGroupTimeoutsElRef {
        EksNodeGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<EksNodeGroupUpdateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
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
            data: std::cell::RefCell::new(EksNodeGroupData {
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
                release_version: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                remote_access: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                taint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_config: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<EksNodeGroupResourcesElRef> {
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
    pub fn launch_template(&self) -> terrars::ListRef<EksNodeGroupLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<EksNodeGroupRemoteAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<EksNodeGroupScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksNodeGroupTimeoutsElRef {
        EksNodeGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<EksNodeGroupUpdateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupResourcesElAutoscalingGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroupResourcesElAutoscalingGroupsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroupResourcesElAutoscalingGroupsEl {
    type O = terrars::BlockAssignable<EksNodeGroupResourcesElAutoscalingGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupResourcesElAutoscalingGroupsEl {}

impl BuildEksNodeGroupResourcesElAutoscalingGroupsEl {
    pub fn build(self) -> EksNodeGroupResourcesElAutoscalingGroupsEl {
        EksNodeGroupResourcesElAutoscalingGroupsEl { name: core::default::Default::default() }
    }
}

pub struct EksNodeGroupResourcesElAutoscalingGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupResourcesElAutoscalingGroupsElRef {
        EksNodeGroupResourcesElAutoscalingGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_groups: std::option::Option<terrars::ListField<EksNodeGroupResourcesElAutoscalingGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_access_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroupResourcesEl {
    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<EksNodeGroupResourcesElAutoscalingGroupsEl>>,
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

impl terrars::ToListMappable for EksNodeGroupResourcesEl {
    type O = terrars::BlockAssignable<EksNodeGroupResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupResourcesEl {}

impl BuildEksNodeGroupResourcesEl {
    pub fn build(self) -> EksNodeGroupResourcesEl {
        EksNodeGroupResourcesEl {
            autoscaling_groups: core::default::Default::default(),
            remote_access_security_group_id: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupResourcesElRef {
        EksNodeGroupResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> terrars::ListRef<EksNodeGroupResourcesElAutoscalingGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_access_security_group_id` after provisioning.\n"]
    pub fn remote_access_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_access_security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    version: terrars::PrimField<std::string::String>,
}

impl EksNodeGroupLaunchTemplateEl {
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

impl terrars::ToListMappable for EksNodeGroupLaunchTemplateEl {
    type O = terrars::BlockAssignable<EksNodeGroupLaunchTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupLaunchTemplateEl {
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildEksNodeGroupLaunchTemplateEl {
    pub fn build(self) -> EksNodeGroupLaunchTemplateEl {
        EksNodeGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: self.version,
        }
    }
}

pub struct EksNodeGroupLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupLaunchTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupLaunchTemplateElRef {
        EksNodeGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupLaunchTemplateElRef {
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
pub struct EksNodeGroupRemoteAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_ssh_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EksNodeGroupRemoteAccessEl {
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

impl terrars::ToListMappable for EksNodeGroupRemoteAccessEl {
    type O = terrars::BlockAssignable<EksNodeGroupRemoteAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupRemoteAccessEl {}

impl BuildEksNodeGroupRemoteAccessEl {
    pub fn build(self) -> EksNodeGroupRemoteAccessEl {
        EksNodeGroupRemoteAccessEl {
            ec2_ssh_key: core::default::Default::default(),
            source_security_group_ids: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupRemoteAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupRemoteAccessElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupRemoteAccessElRef {
        EksNodeGroupRemoteAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupRemoteAccessElRef {
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
pub struct EksNodeGroupScalingConfigEl {
    desired_size: terrars::PrimField<f64>,
    max_size: terrars::PrimField<f64>,
    min_size: terrars::PrimField<f64>,
}

impl EksNodeGroupScalingConfigEl { }

impl terrars::ToListMappable for EksNodeGroupScalingConfigEl {
    type O = terrars::BlockAssignable<EksNodeGroupScalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupScalingConfigEl {
    #[doc = ""]
    pub desired_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_size: terrars::PrimField<f64>,
}

impl BuildEksNodeGroupScalingConfigEl {
    pub fn build(self) -> EksNodeGroupScalingConfigEl {
        EksNodeGroupScalingConfigEl {
            desired_size: self.desired_size,
            max_size: self.max_size,
            min_size: self.min_size,
        }
    }
}

pub struct EksNodeGroupScalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupScalingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupScalingConfigElRef {
        EksNodeGroupScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupScalingConfigElRef {
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
pub struct EksNodeGroupTaintEl {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroupTaintEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksNodeGroupTaintEl {
    type O = terrars::BlockAssignable<EksNodeGroupTaintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupTaintEl {
    #[doc = ""]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildEksNodeGroupTaintEl {
    pub fn build(self) -> EksNodeGroupTaintEl {
        EksNodeGroupTaintEl {
            effect: self.effect,
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupTaintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupTaintElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupTaintElRef {
        EksNodeGroupTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupTaintElRef {
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
pub struct EksNodeGroupTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksNodeGroupTimeoutsEl {
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

impl terrars::ToListMappable for EksNodeGroupTimeoutsEl {
    type O = terrars::BlockAssignable<EksNodeGroupTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupTimeoutsEl {}

impl BuildEksNodeGroupTimeoutsEl {
    pub fn build(self) -> EksNodeGroupTimeoutsEl {
        EksNodeGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupTimeoutsElRef {
        EksNodeGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupTimeoutsElRef {
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
pub struct EksNodeGroupUpdateConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl EksNodeGroupUpdateConfigEl {
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
}

impl terrars::ToListMappable for EksNodeGroupUpdateConfigEl {
    type O = terrars::BlockAssignable<EksNodeGroupUpdateConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupUpdateConfigEl {}

impl BuildEksNodeGroupUpdateConfigEl {
    pub fn build(self) -> EksNodeGroupUpdateConfigEl {
        EksNodeGroupUpdateConfigEl {
            max_unavailable: core::default::Default::default(),
            max_unavailable_percentage: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupUpdateConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksNodeGroupUpdateConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksNodeGroupUpdateConfigElRef {
        EksNodeGroupUpdateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupUpdateConfigElRef {
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
}

#[derive(Serialize, Default)]
struct EksNodeGroupDynamic {
    launch_template: std::option::Option<terrars::DynamicBlock<EksNodeGroupLaunchTemplateEl>>,
    remote_access: std::option::Option<terrars::DynamicBlock<EksNodeGroupRemoteAccessEl>>,
    scaling_config: std::option::Option<terrars::DynamicBlock<EksNodeGroupScalingConfigEl>>,
    taint: std::option::Option<terrars::DynamicBlock<EksNodeGroupTaintEl>>,
    update_config: std::option::Option<terrars::DynamicBlock<EksNodeGroupUpdateConfigEl>>,
}
