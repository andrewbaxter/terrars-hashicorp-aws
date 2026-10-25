#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEmrcontainersVirtualClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    virtual_cluster_id: terrars::PrimField<std::string::String>,
}

struct DataEmrcontainersVirtualCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmrcontainersVirtualClusterData_>,
}

#[derive(Clone)]
pub struct DataEmrcontainersVirtualCluster(std::rc::Rc<DataEmrcontainersVirtualCluster_>);

impl DataEmrcontainersVirtualCluster {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(&self) -> terrars::ListRef<DataEmrcontainersVirtualCluster_ContainerProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_cluster_id` after provisioning.\n"]
    pub fn virtual_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_cluster_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmrcontainersVirtualCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmrcontainersVirtualCluster { }

impl terrars::ToListMappable for DataEmrcontainersVirtualCluster {
    type O = terrars::ListRef<DataEmrcontainersVirtualClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmrcontainersVirtualCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_emrcontainers_virtual_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmrcontainersVirtualCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub virtual_cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEmrcontainersVirtualCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmrcontainersVirtualCluster {
        let out = DataEmrcontainersVirtualCluster(std::rc::Rc::new(DataEmrcontainersVirtualCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmrcontainersVirtualClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_cluster_id: self.virtual_cluster_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmrcontainersVirtualClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrcontainersVirtualClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmrcontainersVirtualClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(&self) -> terrars::ListRef<DataEmrcontainersVirtualCluster_ContainerProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_cluster_id` after provisioning.\n"]
    pub fn virtual_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_cluster_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {
    type O = terrars::BlockAssignable<DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {}

impl BuildDataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {
    pub fn build(self) -> DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {
        DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo {
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfoRef {
        DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmrcontainersVirtualCluster_ContainerProvider_Info {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_info: std::option::Option<terrars::ListField<DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo>>,
}

impl DataEmrcontainersVirtualCluster_ContainerProvider_Info {
    #[doc = "Set the field `eks_info`.\n"]
    pub fn set_eks_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfo>>,
    ) -> Self {
        self.eks_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmrcontainersVirtualCluster_ContainerProvider_Info {
    type O = terrars::BlockAssignable<DataEmrcontainersVirtualCluster_ContainerProvider_Info>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrcontainersVirtualCluster_ContainerProvider_Info {}

impl BuildDataEmrcontainersVirtualCluster_ContainerProvider_Info {
    pub fn build(self) -> DataEmrcontainersVirtualCluster_ContainerProvider_Info {
        DataEmrcontainersVirtualCluster_ContainerProvider_Info { eks_info: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEmrcontainersVirtualCluster_ContainerProvider_InfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrcontainersVirtualCluster_ContainerProvider_InfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmrcontainersVirtualCluster_ContainerProvider_InfoRef {
        DataEmrcontainersVirtualCluster_ContainerProvider_InfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrcontainersVirtualCluster_ContainerProvider_InfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eks_info` after provisioning.\n"]
    pub fn eks_info(&self) -> terrars::ListRef<DataEmrcontainersVirtualCluster_ContainerProvider_Info_EksInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmrcontainersVirtualCluster_ContainerProvider {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info: std::option::Option<terrars::ListField<DataEmrcontainersVirtualCluster_ContainerProvider_Info>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEmrcontainersVirtualCluster_ContainerProvider {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `info`.\n"]
    pub fn set_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEmrcontainersVirtualCluster_ContainerProvider_Info>>,
    ) -> Self {
        self.info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmrcontainersVirtualCluster_ContainerProvider {
    type O = terrars::BlockAssignable<DataEmrcontainersVirtualCluster_ContainerProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrcontainersVirtualCluster_ContainerProvider {}

impl BuildDataEmrcontainersVirtualCluster_ContainerProvider {
    pub fn build(self) -> DataEmrcontainersVirtualCluster_ContainerProvider {
        DataEmrcontainersVirtualCluster_ContainerProvider {
            id: core::default::Default::default(),
            info: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEmrcontainersVirtualCluster_ContainerProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrcontainersVirtualCluster_ContainerProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmrcontainersVirtualCluster_ContainerProviderRef {
        DataEmrcontainersVirtualCluster_ContainerProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrcontainersVirtualCluster_ContainerProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `info` after provisioning.\n"]
    pub fn info(&self) -> terrars::ListRef<DataEmrcontainersVirtualCluster_ContainerProvider_InfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
