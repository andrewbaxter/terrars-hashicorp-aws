use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEksNodeGroupData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    node_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEksNodeGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEksNodeGroupData>,
}

#[derive(Clone)]
pub struct DataEksNodeGroup(std::rc::Rc<DataEksNodeGroup_>);

impl DataEksNodeGroup {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<DataEksNodeGroupRemoteAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataEksNodeGroupResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataEksNodeGroupScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> terrars::ListRef<DataEksNodeGroupTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataEksNodeGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEksNodeGroup { }

impl terrars::ToListMappable for DataEksNodeGroup {
    type O = terrars::ListRef<DataEksNodeGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEksNodeGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_eks_node_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEksNodeGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub node_group_name: terrars::PrimField<std::string::String>,
}

impl BuildDataEksNodeGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEksNodeGroup {
        let out = DataEksNodeGroup(std::rc::Rc::new(DataEksNodeGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEksNodeGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                node_group_name: self.node_group_name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEksNodeGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEksNodeGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<DataEksNodeGroupRemoteAccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataEksNodeGroupResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataEksNodeGroupScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> terrars::ListRef<DataEksNodeGroupTaintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupRemoteAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_ssh_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataEksNodeGroupRemoteAccessEl {
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

impl terrars::ToListMappable for DataEksNodeGroupRemoteAccessEl {
    type O = terrars::BlockAssignable<DataEksNodeGroupRemoteAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupRemoteAccessEl {}

impl BuildDataEksNodeGroupRemoteAccessEl {
    pub fn build(self) -> DataEksNodeGroupRemoteAccessEl {
        DataEksNodeGroupRemoteAccessEl {
            ec2_ssh_key: core::default::Default::default(),
            source_security_group_ids: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupRemoteAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroupRemoteAccessElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroupRemoteAccessElRef {
        DataEksNodeGroupRemoteAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupRemoteAccessElRef {
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
pub struct DataEksNodeGroupResourcesElAutoscalingGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroupResourcesElAutoscalingGroupsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksNodeGroupResourcesElAutoscalingGroupsEl {
    type O = terrars::BlockAssignable<DataEksNodeGroupResourcesElAutoscalingGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupResourcesElAutoscalingGroupsEl {}

impl BuildDataEksNodeGroupResourcesElAutoscalingGroupsEl {
    pub fn build(self) -> DataEksNodeGroupResourcesElAutoscalingGroupsEl {
        DataEksNodeGroupResourcesElAutoscalingGroupsEl { name: core::default::Default::default() }
    }
}

pub struct DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
        DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_groups: std::option::Option<terrars::ListField<DataEksNodeGroupResourcesElAutoscalingGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_access_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroupResourcesEl {
    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksNodeGroupResourcesElAutoscalingGroupsEl>>,
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

impl terrars::ToListMappable for DataEksNodeGroupResourcesEl {
    type O = terrars::BlockAssignable<DataEksNodeGroupResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupResourcesEl {}

impl BuildDataEksNodeGroupResourcesEl {
    pub fn build(self) -> DataEksNodeGroupResourcesEl {
        DataEksNodeGroupResourcesEl {
            autoscaling_groups: core::default::Default::default(),
            remote_access_security_group_id: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroupResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroupResourcesElRef {
        DataEksNodeGroupResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> terrars::ListRef<DataEksNodeGroupResourcesElAutoscalingGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_access_security_group_id` after provisioning.\n"]
    pub fn remote_access_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_access_security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupScalingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksNodeGroupScalingConfigEl {
    #[doc = "Set the field `desired_size`.\n"]
    pub fn set_desired_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_size`.\n"]
    pub fn set_max_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_size`.\n"]
    pub fn set_min_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksNodeGroupScalingConfigEl {
    type O = terrars::BlockAssignable<DataEksNodeGroupScalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupScalingConfigEl {}

impl BuildDataEksNodeGroupScalingConfigEl {
    pub fn build(self) -> DataEksNodeGroupScalingConfigEl {
        DataEksNodeGroupScalingConfigEl {
            desired_size: core::default::Default::default(),
            max_size: core::default::Default::default(),
            min_size: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupScalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroupScalingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroupScalingConfigElRef {
        DataEksNodeGroupScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupScalingConfigElRef {
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
pub struct DataEksNodeGroupTaintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroupTaintsEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksNodeGroupTaintsEl {
    type O = terrars::BlockAssignable<DataEksNodeGroupTaintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupTaintsEl {}

impl BuildDataEksNodeGroupTaintsEl {
    pub fn build(self) -> DataEksNodeGroupTaintsEl {
        DataEksNodeGroupTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupTaintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroupTaintsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroupTaintsElRef {
        DataEksNodeGroupTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupTaintsElRef {
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
