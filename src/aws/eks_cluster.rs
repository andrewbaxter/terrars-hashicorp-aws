use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksClusterData {
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
    enabled_cluster_log_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<EksClusterEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_network_config: std::option::Option<std::vec::Vec<EksClusterKubernetesNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outpost_config: std::option::Option<std::vec::Vec<EksClusterOutpostConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EksClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<EksClusterVpcConfigEl>>,
    dynamic: EksClusterDynamic,
}

struct EksCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EksClusterData>,
}

#[derive(Clone)]
pub struct EksCluster(std::rc::Rc<EksCluster_>);

impl EksCluster {
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

    #[doc = "Set the field `enabled_cluster_log_types`.\n"]
    pub fn set_enabled_cluster_log_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_cluster_log_types = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksClusterEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes_network_config`.\n"]
    pub fn set_kubernetes_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksClusterKubernetesNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kubernetes_network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kubernetes_network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outpost_config`.\n"]
    pub fn set_outpost_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksClusterOutpostConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outpost_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outpost_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EksClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<EksClusterVpcConfigEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::ListRef<EksClusterCertificateAuthorityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<EksClusterIdentityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<EksClusterEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> terrars::ListRef<EksClusterKubernetesNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> terrars::ListRef<EksClusterOutpostConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksClusterTimeoutsElRef {
        EksClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<EksClusterVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for EksCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EksCluster { }

impl terrars::ToListMappable for EksCluster {
    type O = terrars::ListRef<EksClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EksCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_eks_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildEksCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> EksCluster {
        let out = EksCluster(std::rc::Rc::new(EksCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EksClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enabled_cluster_log_types: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                kubernetes_network_config: core::default::Default::default(),
                outpost_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EksClusterRef {
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

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::ListRef<EksClusterCertificateAuthorityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<EksClusterIdentityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<EksClusterEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> terrars::ListRef<EksClusterKubernetesNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> terrars::ListRef<EksClusterOutpostConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksClusterTimeoutsElRef {
        EksClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<EksClusterVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EksClusterCertificateAuthorityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksClusterCertificateAuthorityEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksClusterCertificateAuthorityEl {
    type O = terrars::BlockAssignable<EksClusterCertificateAuthorityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterCertificateAuthorityEl {}

impl BuildEksClusterCertificateAuthorityEl {
    pub fn build(self) -> EksClusterCertificateAuthorityEl {
        EksClusterCertificateAuthorityEl { data: core::default::Default::default() }
    }
}

pub struct EksClusterCertificateAuthorityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterCertificateAuthorityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterCertificateAuthorityElRef {
        EksClusterCertificateAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterCertificateAuthorityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterIdentityElOidcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksClusterIdentityElOidcEl {
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksClusterIdentityElOidcEl {
    type O = terrars::BlockAssignable<EksClusterIdentityElOidcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterIdentityElOidcEl {}

impl BuildEksClusterIdentityElOidcEl {
    pub fn build(self) -> EksClusterIdentityElOidcEl {
        EksClusterIdentityElOidcEl { issuer: core::default::Default::default() }
    }
}

pub struct EksClusterIdentityElOidcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterIdentityElOidcElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterIdentityElOidcElRef {
        EksClusterIdentityElOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterIdentityElOidcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterIdentityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<terrars::ListField<EksClusterIdentityElOidcEl>>,
}

impl EksClusterIdentityEl {
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<terrars::ListField<EksClusterIdentityElOidcEl>>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksClusterIdentityEl {
    type O = terrars::BlockAssignable<EksClusterIdentityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterIdentityEl {}

impl BuildEksClusterIdentityEl {
    pub fn build(self) -> EksClusterIdentityEl {
        EksClusterIdentityEl { oidc: core::default::Default::default() }
    }
}

pub struct EksClusterIdentityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterIdentityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterIdentityElRef {
        EksClusterIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterIdentityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<EksClusterIdentityElOidcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterEncryptionConfigElProviderEl {
    key_arn: terrars::PrimField<std::string::String>,
}

impl EksClusterEncryptionConfigElProviderEl { }

impl terrars::ToListMappable for EksClusterEncryptionConfigElProviderEl {
    type O = terrars::BlockAssignable<EksClusterEncryptionConfigElProviderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterEncryptionConfigElProviderEl {
    #[doc = ""]
    pub key_arn: terrars::PrimField<std::string::String>,
}

impl BuildEksClusterEncryptionConfigElProviderEl {
    pub fn build(self) -> EksClusterEncryptionConfigElProviderEl {
        EksClusterEncryptionConfigElProviderEl { key_arn: self.key_arn }
    }
}

pub struct EksClusterEncryptionConfigElProviderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterEncryptionConfigElProviderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterEncryptionConfigElProviderElRef {
        EksClusterEncryptionConfigElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterEncryptionConfigElProviderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_arn` after provisioning.\n"]
    pub fn key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterEncryptionConfigElDynamic {
    provider: std::option::Option<terrars::DynamicBlock<EksClusterEncryptionConfigElProviderEl>>,
}

#[derive(Serialize)]
pub struct EksClusterEncryptionConfigEl {
    resources: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::vec::Vec<EksClusterEncryptionConfigElProviderEl>>,
    dynamic: EksClusterEncryptionConfigElDynamic,
}

impl EksClusterEncryptionConfigEl {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksClusterEncryptionConfigElProviderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksClusterEncryptionConfigEl {
    type O = terrars::BlockAssignable<EksClusterEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterEncryptionConfigEl {
    #[doc = ""]
    pub resources: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksClusterEncryptionConfigEl {
    pub fn build(self) -> EksClusterEncryptionConfigEl {
        EksClusterEncryptionConfigEl {
            resources: self.resources,
            provider: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EksClusterEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterEncryptionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterEncryptionConfigElRef {
        EksClusterEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<EksClusterEncryptionConfigElProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterKubernetesNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_ipv4_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksClusterKubernetesNetworkConfigEl {
    #[doc = "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_ipv4_cidr`.\n"]
    pub fn set_service_ipv4_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_ipv4_cidr = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksClusterKubernetesNetworkConfigEl {
    type O = terrars::BlockAssignable<EksClusterKubernetesNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterKubernetesNetworkConfigEl {}

impl BuildEksClusterKubernetesNetworkConfigEl {
    pub fn build(self) -> EksClusterKubernetesNetworkConfigEl {
        EksClusterKubernetesNetworkConfigEl {
            ip_family: core::default::Default::default(),
            service_ipv4_cidr: core::default::Default::default(),
        }
    }
}

pub struct EksClusterKubernetesNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterKubernetesNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterKubernetesNetworkConfigElRef {
        EksClusterKubernetesNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterKubernetesNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }

    #[doc = "Get a reference to the value of field `service_ipv4_cidr` after provisioning.\n"]
    pub fn service_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_ipv4_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `service_ipv6_cidr` after provisioning.\n"]
    pub fn service_ipv6_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_ipv6_cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterOutpostConfigElControlPlanePlacementEl {
    group_name: terrars::PrimField<std::string::String>,
}

impl EksClusterOutpostConfigElControlPlanePlacementEl { }

impl terrars::ToListMappable for EksClusterOutpostConfigElControlPlanePlacementEl {
    type O = terrars::BlockAssignable<EksClusterOutpostConfigElControlPlanePlacementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterOutpostConfigElControlPlanePlacementEl {
    #[doc = ""]
    pub group_name: terrars::PrimField<std::string::String>,
}

impl BuildEksClusterOutpostConfigElControlPlanePlacementEl {
    pub fn build(self) -> EksClusterOutpostConfigElControlPlanePlacementEl {
        EksClusterOutpostConfigElControlPlanePlacementEl { group_name: self.group_name }
    }
}

pub struct EksClusterOutpostConfigElControlPlanePlacementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterOutpostConfigElControlPlanePlacementElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksClusterOutpostConfigElControlPlanePlacementElRef {
        EksClusterOutpostConfigElControlPlanePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterOutpostConfigElControlPlanePlacementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterOutpostConfigElDynamic {
    control_plane_placement: std
    ::option
    ::Option<terrars::DynamicBlock<EksClusterOutpostConfigElControlPlanePlacementEl>>,
}

#[derive(Serialize)]
pub struct EksClusterOutpostConfigEl {
    control_plane_instance_type: terrars::PrimField<std::string::String>,
    outpost_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_placement: std::option::Option<std::vec::Vec<EksClusterOutpostConfigElControlPlanePlacementEl>>,
    dynamic: EksClusterOutpostConfigElDynamic,
}

impl EksClusterOutpostConfigEl {
    #[doc = "Set the field `control_plane_placement`.\n"]
    pub fn set_control_plane_placement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksClusterOutpostConfigElControlPlanePlacementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_plane_placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_placement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksClusterOutpostConfigEl {
    type O = terrars::BlockAssignable<EksClusterOutpostConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterOutpostConfigEl {
    #[doc = ""]
    pub control_plane_instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub outpost_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksClusterOutpostConfigEl {
    pub fn build(self) -> EksClusterOutpostConfigEl {
        EksClusterOutpostConfigEl {
            control_plane_instance_type: self.control_plane_instance_type,
            outpost_arns: self.outpost_arns,
            control_plane_placement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EksClusterOutpostConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterOutpostConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterOutpostConfigElRef {
        EksClusterOutpostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterOutpostConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_instance_type` after provisioning.\n"]
    pub fn control_plane_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `outpost_arns` after provisioning.\n"]
    pub fn outpost_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.outpost_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_placement` after provisioning.\n"]
    pub fn control_plane_placement(&self) -> terrars::ListRef<EksClusterOutpostConfigElControlPlanePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_placement", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksClusterTimeoutsEl {
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

impl terrars::ToListMappable for EksClusterTimeoutsEl {
    type O = terrars::BlockAssignable<EksClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterTimeoutsEl {}

impl BuildEksClusterTimeoutsEl {
    pub fn build(self) -> EksClusterTimeoutsEl {
        EksClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EksClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterTimeoutsElRef {
        EksClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterTimeoutsElRef {
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
pub struct EksClusterVpcConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_private_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_public_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access_cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EksClusterVpcConfigEl {
    #[doc = "Set the field `endpoint_private_access`.\n"]
    pub fn set_endpoint_private_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.endpoint_private_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_public_access`.\n"]
    pub fn set_endpoint_public_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.endpoint_public_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_access_cidrs`.\n"]
    pub fn set_public_access_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.public_access_cidrs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksClusterVpcConfigEl {
    type O = terrars::BlockAssignable<EksClusterVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterVpcConfigEl {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksClusterVpcConfigEl {
    pub fn build(self) -> EksClusterVpcConfigEl {
        EksClusterVpcConfigEl {
            endpoint_private_access: core::default::Default::default(),
            endpoint_public_access: core::default::Default::default(),
            public_access_cidrs: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct EksClusterVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksClusterVpcConfigElRef {
        EksClusterVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterVpcConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_security_group_id` after provisioning.\n"]
    pub fn cluster_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_private_access` after provisioning.\n"]
    pub fn endpoint_private_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_private_access", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_public_access` after provisioning.\n"]
    pub fn endpoint_public_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_public_access", self.base))
    }

    #[doc = "Get a reference to the value of field `public_access_cidrs` after provisioning.\n"]
    pub fn public_access_cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.public_access_cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<EksClusterEncryptionConfigEl>>,
    kubernetes_network_config: std::option::Option<terrars::DynamicBlock<EksClusterKubernetesNetworkConfigEl>>,
    outpost_config: std::option::Option<terrars::DynamicBlock<EksClusterOutpostConfigEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<EksClusterVpcConfigEl>>,
}
