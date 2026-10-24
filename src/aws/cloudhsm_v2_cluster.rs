use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudhsmV2ClusterData {
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
    hsm_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_backup_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudhsmV2ClusterTimeoutsEl>,
}

struct CloudhsmV2Cluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudhsmV2ClusterData>,
}

#[derive(Clone)]
pub struct CloudhsmV2Cluster(std::rc::Rc<CloudhsmV2Cluster_>);

impl CloudhsmV2Cluster {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_backup_identifier`.\n"]
    pub fn set_source_backup_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().source_backup_identifier = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudhsmV2ClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> terrars::ListRef<CloudhsmV2ClusterClusterCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hsm_type` after provisioning.\n"]
    pub fn hsm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hsm_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_backup_identifier` after provisioning.\n"]
    pub fn source_backup_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backup_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudhsmV2ClusterTimeoutsElRef {
        CloudhsmV2ClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudhsmV2Cluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudhsmV2Cluster { }

impl terrars::ToListMappable for CloudhsmV2Cluster {
    type O = terrars::ListRef<CloudhsmV2ClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudhsmV2Cluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudhsm_v2_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudhsmV2Cluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub hsm_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudhsmV2Cluster {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudhsmV2Cluster {
        let out = CloudhsmV2Cluster(std::rc::Rc::new(CloudhsmV2Cluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudhsmV2ClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                hsm_type: self.hsm_type,
                id: core::default::Default::default(),
                source_backup_identifier: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudhsmV2ClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudhsmV2ClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudhsmV2ClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> terrars::ListRef<CloudhsmV2ClusterClusterCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hsm_type` after provisioning.\n"]
    pub fn hsm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hsm_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_backup_identifier` after provisioning.\n"]
    pub fn source_backup_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_backup_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudhsmV2ClusterTimeoutsElRef {
        CloudhsmV2ClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudhsmV2ClusterClusterCertificatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_hardware_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_csr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hsm_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer_hardware_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudhsmV2ClusterClusterCertificatesEl {
    #[doc = "Set the field `aws_hardware_certificate`.\n"]
    pub fn set_aws_hardware_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aws_hardware_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_certificate`.\n"]
    pub fn set_cluster_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_csr`.\n"]
    pub fn set_cluster_csr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_csr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hsm_certificate`.\n"]
    pub fn set_hsm_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hsm_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer_hardware_certificate`.\n"]
    pub fn set_manufacturer_hardware_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.manufacturer_hardware_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudhsmV2ClusterClusterCertificatesEl {
    type O = terrars::BlockAssignable<CloudhsmV2ClusterClusterCertificatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudhsmV2ClusterClusterCertificatesEl {}

impl BuildCloudhsmV2ClusterClusterCertificatesEl {
    pub fn build(self) -> CloudhsmV2ClusterClusterCertificatesEl {
        CloudhsmV2ClusterClusterCertificatesEl {
            aws_hardware_certificate: core::default::Default::default(),
            cluster_certificate: core::default::Default::default(),
            cluster_csr: core::default::Default::default(),
            hsm_certificate: core::default::Default::default(),
            manufacturer_hardware_certificate: core::default::Default::default(),
        }
    }
}

pub struct CloudhsmV2ClusterClusterCertificatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudhsmV2ClusterClusterCertificatesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudhsmV2ClusterClusterCertificatesElRef {
        CloudhsmV2ClusterClusterCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudhsmV2ClusterClusterCertificatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_hardware_certificate` after provisioning.\n"]
    pub fn aws_hardware_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_hardware_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_certificate` after provisioning.\n"]
    pub fn cluster_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_csr` after provisioning.\n"]
    pub fn cluster_csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_csr", self.base))
    }

    #[doc = "Get a reference to the value of field `hsm_certificate` after provisioning.\n"]
    pub fn hsm_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hsm_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer_hardware_certificate` after provisioning.\n"]
    pub fn manufacturer_hardware_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manufacturer_hardware_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudhsmV2ClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudhsmV2ClusterTimeoutsEl {
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

impl terrars::ToListMappable for CloudhsmV2ClusterTimeoutsEl {
    type O = terrars::BlockAssignable<CloudhsmV2ClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudhsmV2ClusterTimeoutsEl {}

impl BuildCloudhsmV2ClusterTimeoutsEl {
    pub fn build(self) -> CloudhsmV2ClusterTimeoutsEl {
        CloudhsmV2ClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudhsmV2ClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudhsmV2ClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudhsmV2ClusterTimeoutsElRef {
        CloudhsmV2ClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudhsmV2ClusterTimeoutsElRef {
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
