#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEksNodeGroupData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEksNodeGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEksNodeGroupData_>,
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

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<DataEksNodeGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<DataEksNodeGroup_RemoteAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataEksNodeGroup_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataEksNodeGroup_ScalingConfigRef> {
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
    pub fn taints(&self) -> terrars::ListRef<DataEksNodeGroup_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<DataEksNodeGroup_UpdateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool_config` after provisioning.\n"]
    pub fn warm_pool_config(&self) -> terrars::ListRef<DataEksNodeGroup_WarmPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool_config", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataEksNodeGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                node_group_name: self.node_group_name,
                region: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<DataEksNodeGroup_LaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> terrars::ListRef<DataEksNodeGroup_RemoteAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataEksNodeGroup_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataEksNodeGroup_ScalingConfigRef> {
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
    pub fn taints(&self) -> terrars::ListRef<DataEksNodeGroup_TaintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.taints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<DataEksNodeGroup_UpdateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_pool_config` after provisioning.\n"]
    pub fn warm_pool_config(&self) -> terrars::ListRef<DataEksNodeGroup_WarmPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_pool_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroup_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroup_LaunchTemplate {
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksNodeGroup_LaunchTemplate {
    type O = terrars::BlockAssignable<DataEksNodeGroup_LaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_LaunchTemplate {}

impl BuildDataEksNodeGroup_LaunchTemplate {
    pub fn build(self) -> DataEksNodeGroup_LaunchTemplate {
        DataEksNodeGroup_LaunchTemplate {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_LaunchTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_LaunchTemplateRef {
        DataEksNodeGroup_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_LaunchTemplateRef {
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
pub struct DataEksNodeGroup_RemoteAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_ssh_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataEksNodeGroup_RemoteAccess {
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

impl terrars::ToListMappable for DataEksNodeGroup_RemoteAccess {
    type O = terrars::BlockAssignable<DataEksNodeGroup_RemoteAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_RemoteAccess {}

impl BuildDataEksNodeGroup_RemoteAccess {
    pub fn build(self) -> DataEksNodeGroup_RemoteAccess {
        DataEksNodeGroup_RemoteAccess {
            ec2_ssh_key: core::default::Default::default(),
            source_security_group_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_RemoteAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_RemoteAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_RemoteAccessRef {
        DataEksNodeGroup_RemoteAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_RemoteAccessRef {
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
pub struct DataEksNodeGroup_Resources_AutoscalingGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroup_Resources_AutoscalingGroups {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksNodeGroup_Resources_AutoscalingGroups {
    type O = terrars::BlockAssignable<DataEksNodeGroup_Resources_AutoscalingGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_Resources_AutoscalingGroups {}

impl BuildDataEksNodeGroup_Resources_AutoscalingGroups {
    pub fn build(self) -> DataEksNodeGroup_Resources_AutoscalingGroups {
        DataEksNodeGroup_Resources_AutoscalingGroups { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_Resources_AutoscalingGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_Resources_AutoscalingGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksNodeGroup_Resources_AutoscalingGroupsRef {
        DataEksNodeGroup_Resources_AutoscalingGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_Resources_AutoscalingGroupsRef {
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
pub struct DataEksNodeGroup_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_groups: std::option::Option<terrars::ListField<DataEksNodeGroup_Resources_AutoscalingGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_access_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroup_Resources {
    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksNodeGroup_Resources_AutoscalingGroups>>,
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

impl terrars::ToListMappable for DataEksNodeGroup_Resources {
    type O = terrars::BlockAssignable<DataEksNodeGroup_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_Resources {}

impl BuildDataEksNodeGroup_Resources {
    pub fn build(self) -> DataEksNodeGroup_Resources {
        DataEksNodeGroup_Resources {
            autoscaling_groups: core::default::Default::default(),
            remote_access_security_group_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_ResourcesRef {
        DataEksNodeGroup_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> terrars::ListRef<DataEksNodeGroup_Resources_AutoscalingGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_access_security_group_id` after provisioning.\n"]
    pub fn remote_access_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_access_security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroup_ScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksNodeGroup_ScalingConfig {
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

impl terrars::ToListMappable for DataEksNodeGroup_ScalingConfig {
    type O = terrars::BlockAssignable<DataEksNodeGroup_ScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_ScalingConfig {}

impl BuildDataEksNodeGroup_ScalingConfig {
    pub fn build(self) -> DataEksNodeGroup_ScalingConfig {
        DataEksNodeGroup_ScalingConfig {
            desired_size: core::default::Default::default(),
            max_size: core::default::Default::default(),
            min_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_ScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_ScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_ScalingConfigRef {
        DataEksNodeGroup_ScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_ScalingConfigRef {
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
pub struct DataEksNodeGroup_Taints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroup_Taints {
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

impl terrars::ToListMappable for DataEksNodeGroup_Taints {
    type O = terrars::BlockAssignable<DataEksNodeGroup_Taints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_Taints {}

impl BuildDataEksNodeGroup_Taints {
    pub fn build(self) -> DataEksNodeGroup_Taints {
        DataEksNodeGroup_Taints {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_TaintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_TaintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_TaintsRef {
        DataEksNodeGroup_TaintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_TaintsRef {
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
pub struct DataEksNodeGroup_UpdateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksNodeGroup_UpdateConfig {
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

impl terrars::ToListMappable for DataEksNodeGroup_UpdateConfig {
    type O = terrars::BlockAssignable<DataEksNodeGroup_UpdateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_UpdateConfig {}

impl BuildDataEksNodeGroup_UpdateConfig {
    pub fn build(self) -> DataEksNodeGroup_UpdateConfig {
        DataEksNodeGroup_UpdateConfig {
            max_unavailable: core::default::Default::default(),
            max_unavailable_percentage: core::default::Default::default(),
            update_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_UpdateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_UpdateConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_UpdateConfigRef {
        DataEksNodeGroup_UpdateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_UpdateConfigRef {
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
pub struct DataEksNodeGroup_WarmPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_group_prepared_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reuse_on_scale_in: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEksNodeGroup_WarmPoolConfig {
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

impl terrars::ToListMappable for DataEksNodeGroup_WarmPoolConfig {
    type O = terrars::BlockAssignable<DataEksNodeGroup_WarmPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroup_WarmPoolConfig {}

impl BuildDataEksNodeGroup_WarmPoolConfig {
    pub fn build(self) -> DataEksNodeGroup_WarmPoolConfig {
        DataEksNodeGroup_WarmPoolConfig {
            max_group_prepared_capacity: core::default::Default::default(),
            min_size: core::default::Default::default(),
            pool_state: core::default::Default::default(),
            reuse_on_scale_in: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksNodeGroup_WarmPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksNodeGroup_WarmPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksNodeGroup_WarmPoolConfigRef {
        DataEksNodeGroup_WarmPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroup_WarmPoolConfigRef {
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
