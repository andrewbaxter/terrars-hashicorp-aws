use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEksClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataEksCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEksClusterData>,
}

#[derive(Clone)]
pub struct DataEksCluster(Rc<DataEksCluster_>);

impl DataEksCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<DataEksClusterCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<DataEksClusterIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> ListRef<DataEksClusterKubernetesNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> ListRef<DataEksClusterOutpostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataEksClusterVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl Referable for DataEksCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEksCluster { }

impl ToListMappable for DataEksCluster {
    type O = ListRef<DataEksClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEksCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_eks_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEksCluster {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataEksCluster {
    pub fn build(self, stack: &mut Stack) -> DataEksCluster {
        let out = DataEksCluster(Rc::new(DataEksCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEksClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEksClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEksClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<DataEksClusterCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<DataEksClusterIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> ListRef<DataEksClusterKubernetesNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> ListRef<DataEksClusterOutpostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataEksClusterVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterCertificateAuthorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
}

impl DataEksClusterCertificateAuthorityEl {
    #[doc= "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterCertificateAuthorityEl {
    type O = BlockAssignable<DataEksClusterCertificateAuthorityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterCertificateAuthorityEl {}

impl BuildDataEksClusterCertificateAuthorityEl {
    pub fn build(self) -> DataEksClusterCertificateAuthorityEl {
        DataEksClusterCertificateAuthorityEl { data: core::default::Default::default() }
    }
}

pub struct DataEksClusterCertificateAuthorityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterCertificateAuthorityElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterCertificateAuthorityElRef {
        DataEksClusterCertificateAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterCertificateAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterIdentityElOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl DataEksClusterIdentityElOidcEl {
    #[doc= "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterIdentityElOidcEl {
    type O = BlockAssignable<DataEksClusterIdentityElOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterIdentityElOidcEl {}

impl BuildDataEksClusterIdentityElOidcEl {
    pub fn build(self) -> DataEksClusterIdentityElOidcEl {
        DataEksClusterIdentityElOidcEl { issuer: core::default::Default::default() }
    }
}

pub struct DataEksClusterIdentityElOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterIdentityElOidcElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterIdentityElOidcElRef {
        DataEksClusterIdentityElOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterIdentityElOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc: Option<ListField<DataEksClusterIdentityElOidcEl>>,
}

impl DataEksClusterIdentityEl {
    #[doc= "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl Into<ListField<DataEksClusterIdentityElOidcEl>>) -> Self {
        self.oidc = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterIdentityEl {
    type O = BlockAssignable<DataEksClusterIdentityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterIdentityEl {}

impl BuildDataEksClusterIdentityEl {
    pub fn build(self) -> DataEksClusterIdentityEl {
        DataEksClusterIdentityEl { oidc: core::default::Default::default() }
    }
}

pub struct DataEksClusterIdentityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterIdentityElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterIdentityElRef {
        DataEksClusterIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<DataEksClusterIdentityElOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterKubernetesNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_ipv4_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_ipv6_cidr: Option<PrimField<String>>,
}

impl DataEksClusterKubernetesNetworkConfigEl {
    #[doc= "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_family = Some(v.into());
        self
    }

    #[doc= "Set the field `service_ipv4_cidr`.\n"]
    pub fn set_service_ipv4_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_ipv4_cidr = Some(v.into());
        self
    }

    #[doc= "Set the field `service_ipv6_cidr`.\n"]
    pub fn set_service_ipv6_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_ipv6_cidr = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterKubernetesNetworkConfigEl {
    type O = BlockAssignable<DataEksClusterKubernetesNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterKubernetesNetworkConfigEl {}

impl BuildDataEksClusterKubernetesNetworkConfigEl {
    pub fn build(self) -> DataEksClusterKubernetesNetworkConfigEl {
        DataEksClusterKubernetesNetworkConfigEl {
            ip_family: core::default::Default::default(),
            service_ipv4_cidr: core::default::Default::default(),
            service_ipv6_cidr: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterKubernetesNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterKubernetesNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterKubernetesNetworkConfigElRef {
        DataEksClusterKubernetesNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterKubernetesNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }

    #[doc= "Get a reference to the value of field `service_ipv4_cidr` after provisioning.\n"]
    pub fn service_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_ipv4_cidr", self.base))
    }

    #[doc= "Get a reference to the value of field `service_ipv6_cidr` after provisioning.\n"]
    pub fn service_ipv6_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_ipv6_cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterOutpostConfigElControlPlanePlacementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl DataEksClusterOutpostConfigElControlPlanePlacementEl {
    #[doc= "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterOutpostConfigElControlPlanePlacementEl {
    type O = BlockAssignable<DataEksClusterOutpostConfigElControlPlanePlacementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterOutpostConfigElControlPlanePlacementEl {}

impl BuildDataEksClusterOutpostConfigElControlPlanePlacementEl {
    pub fn build(self) -> DataEksClusterOutpostConfigElControlPlanePlacementEl {
        DataEksClusterOutpostConfigElControlPlanePlacementEl { group_name: core::default::Default::default() }
    }
}

pub struct DataEksClusterOutpostConfigElControlPlanePlacementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterOutpostConfigElControlPlanePlacementElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterOutpostConfigElControlPlanePlacementElRef {
        DataEksClusterOutpostConfigElControlPlanePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterOutpostConfigElControlPlanePlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterOutpostConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_placement: Option<ListField<DataEksClusterOutpostConfigElControlPlanePlacementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outpost_arns: Option<SetField<PrimField<String>>>,
}

impl DataEksClusterOutpostConfigEl {
    #[doc= "Set the field `control_plane_instance_type`.\n"]
    pub fn set_control_plane_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.control_plane_instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `control_plane_placement`.\n"]
    pub fn set_control_plane_placement(
        mut self,
        v: impl Into<ListField<DataEksClusterOutpostConfigElControlPlanePlacementEl>>,
    ) -> Self {
        self.control_plane_placement = Some(v.into());
        self
    }

    #[doc= "Set the field `outpost_arns`.\n"]
    pub fn set_outpost_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.outpost_arns = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterOutpostConfigEl {
    type O = BlockAssignable<DataEksClusterOutpostConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterOutpostConfigEl {}

impl BuildDataEksClusterOutpostConfigEl {
    pub fn build(self) -> DataEksClusterOutpostConfigEl {
        DataEksClusterOutpostConfigEl {
            control_plane_instance_type: core::default::Default::default(),
            control_plane_placement: core::default::Default::default(),
            outpost_arns: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterOutpostConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterOutpostConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterOutpostConfigElRef {
        DataEksClusterOutpostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterOutpostConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `control_plane_instance_type` after provisioning.\n"]
    pub fn control_plane_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_plane_instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `control_plane_placement` after provisioning.\n"]
    pub fn control_plane_placement(&self) -> ListRef<DataEksClusterOutpostConfigElControlPlanePlacementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.control_plane_placement", self.base))
    }

    #[doc= "Get a reference to the value of field `outpost_arns` after provisioning.\n"]
    pub fn outpost_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.outpost_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_security_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_private_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_public_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access_cidrs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl DataEksClusterVpcConfigEl {
    #[doc= "Set the field `cluster_security_group_id`.\n"]
    pub fn set_cluster_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_security_group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint_private_access`.\n"]
    pub fn set_endpoint_private_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.endpoint_private_access = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint_public_access`.\n"]
    pub fn set_endpoint_public_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.endpoint_public_access = Some(v.into());
        self
    }

    #[doc= "Set the field `public_access_cidrs`.\n"]
    pub fn set_public_access_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.public_access_cidrs = Some(v.into());
        self
    }

    #[doc= "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterVpcConfigEl {
    type O = BlockAssignable<DataEksClusterVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVpcConfigEl {}

impl BuildDataEksClusterVpcConfigEl {
    pub fn build(self) -> DataEksClusterVpcConfigEl {
        DataEksClusterVpcConfigEl {
            cluster_security_group_id: core::default::Default::default(),
            endpoint_private_access: core::default::Default::default(),
            endpoint_public_access: core::default::Default::default(),
            public_access_cidrs: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterVpcConfigElRef {
        DataEksClusterVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_security_group_id` after provisioning.\n"]
    pub fn cluster_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_security_group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint_private_access` after provisioning.\n"]
    pub fn endpoint_private_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_private_access", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint_public_access` after provisioning.\n"]
    pub fn endpoint_public_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_public_access", self.base))
    }

    #[doc= "Get a reference to the value of field `public_access_cidrs` after provisioning.\n"]
    pub fn public_access_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.public_access_cidrs", self.base))
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
