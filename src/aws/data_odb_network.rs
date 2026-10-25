#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbNetworkData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbNetwork_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbNetworkData_>,
}

#[derive(Clone)]
pub struct DataOdbNetwork(std::rc::Rc<DataOdbNetwork_>);

impl DataOdbNetwork {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nThe availability zone where the ODB network is located."]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the ODB network is located."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\n The CIDR range of the backup subnet for the ODB network."]
    pub fn backup_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_subnet_cidr` after provisioning.\nThe CIDR notation for the network resource."]
    pub fn client_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the ODB network was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_domain_name` after provisioning.\nThe name of the custom domain that the network is located."]
    pub fn custom_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_dns_prefix` after provisioning.\nThe default DNS prefix for the network resource."]
    pub fn default_dns_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_dns_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the network resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ec2_placement_group_ids` after provisioning.\nA list of EC2 placement group IDs associated with the ODB network."]
    pub fn ec2_placement_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_placement_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_services` after provisioning.\nThe managed services configuration for the ODB network."]
    pub fn managed_services(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_dns_forwarding_configs` after provisioning.\nThe DNS resolver endpoint in OCI for forwarding DNS queries for the ociPrivateZone domain."]
    pub fn oci_dns_forwarding_configs(&self) -> terrars::ListRef<DataOdbNetwork_OciDnsForwardingConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oci_dns_forwarding_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\nThe unique identifier of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_url` after provisioning.\nThe URL of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the ODB network."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_id` after provisioning.\nThe unique identifier  Oracle Cloud ID (OCID) of the OCI VCN for the ODB network."]
    pub fn oci_vcn_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_url` after provisioning.\nThe URL of the OCI VCN for the ODB network."]
    pub fn oci_vcn_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peered_cidrs` after provisioning.\nThe list of CIDR ranges from the peered VPC that are allowed access to the ODB network. Please refer odb network peering documentation."]
    pub fn peered_cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.peered_cidrs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the ODB network, expressed as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the network resource."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the ODB network."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbNetwork {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbNetwork { }

impl terrars::ToListMappable for DataOdbNetwork {
    type O = terrars::ListRef<DataOdbNetworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbNetwork_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_network".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbNetwork {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataOdbNetwork {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbNetwork {
        let out = DataOdbNetwork(std::rc::Rc::new(DataOdbNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbNetworkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbNetworkRef {
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

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nThe availability zone where the ODB network is located."]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the ODB network is located."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\n The CIDR range of the backup subnet for the ODB network."]
    pub fn backup_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_subnet_cidr` after provisioning.\nThe CIDR notation for the network resource."]
    pub fn client_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the ODB network was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_domain_name` after provisioning.\nThe name of the custom domain that the network is located."]
    pub fn custom_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_dns_prefix` after provisioning.\nThe default DNS prefix for the network resource."]
    pub fn default_dns_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_dns_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the network resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ec2_placement_group_ids` after provisioning.\nA list of EC2 placement group IDs associated with the ODB network."]
    pub fn ec2_placement_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_placement_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_services` after provisioning.\nThe managed services configuration for the ODB network."]
    pub fn managed_services(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_dns_forwarding_configs` after provisioning.\nThe DNS resolver endpoint in OCI for forwarding DNS queries for the ociPrivateZone domain."]
    pub fn oci_dns_forwarding_configs(&self) -> terrars::ListRef<DataOdbNetwork_OciDnsForwardingConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oci_dns_forwarding_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\nThe unique identifier of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_url` after provisioning.\nThe URL of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the ODB network."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_id` after provisioning.\nThe unique identifier  Oracle Cloud ID (OCID) of the OCI VCN for the ODB network."]
    pub fn oci_vcn_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_url` after provisioning.\nThe URL of the OCI VCN for the ODB network."]
    pub fn oci_vcn_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peered_cidrs` after provisioning.\nThe list of CIDR ranges from the peered VPC that are allowed access to the ODB network. Please refer odb network peering documentation."]
    pub fn peered_cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.peered_cidrs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the ODB network, expressed as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the network resource."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the ODB network."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {
    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {}

impl BuildDataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {
        DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess {
            ipv4_addresses: core::default::Default::default(),
            region: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccessRef {
        DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_KmsAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_policy_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_KmsAccess {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_policy_document`.\n"]
    pub fn set_kms_policy_document(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_policy_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_KmsAccess {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_KmsAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_KmsAccess {}

impl BuildDataOdbNetwork_ManagedServices_KmsAccess {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_KmsAccess {
        DataOdbNetwork_ManagedServices_KmsAccess {
            domain_name: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            kms_policy_document: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_KmsAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_KmsAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbNetwork_ManagedServices_KmsAccessRef {
        DataOdbNetwork_ManagedServices_KmsAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_KmsAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_policy_document` after provisioning.\n"]
    pub fn kms_policy_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_policy_document", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_ManagedS3BackupAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_ManagedS3BackupAccess {
    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_ManagedS3BackupAccess {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_ManagedS3BackupAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_ManagedS3BackupAccess {}

impl BuildDataOdbNetwork_ManagedServices_ManagedS3BackupAccess {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_ManagedS3BackupAccess {
        DataOdbNetwork_ManagedServices_ManagedS3BackupAccess {
            ipv4_addresses: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_ManagedS3BackupAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_ManagedS3BackupAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbNetwork_ManagedServices_ManagedS3BackupAccessRef {
        DataOdbNetwork_ManagedServices_ManagedS3BackupAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_ManagedS3BackupAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_S3Access {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_policy_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_S3Access {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_policy_document`.\n"]
    pub fn set_s3_policy_document(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_policy_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_S3Access {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_S3Access>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_S3Access {}

impl BuildDataOdbNetwork_ManagedServices_S3Access {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_S3Access {
        DataOdbNetwork_ManagedServices_S3Access {
            domain_name: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            s3_policy_document: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_S3AccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_S3AccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbNetwork_ManagedServices_S3AccessRef {
        DataOdbNetwork_ManagedServices_S3AccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_S3AccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_policy_document` after provisioning.\n"]
    pub fn s3_policy_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_policy_document", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {
    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_type`.\n"]
    pub fn set_vpc_endpoint_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_endpoint_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {}

impl BuildDataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {
        DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint {
            vpc_endpoint_id: core::default::Default::default(),
            vpc_endpoint_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_ServiceNetworkEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_ServiceNetworkEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbNetwork_ManagedServices_ServiceNetworkEndpointRef {
        DataOdbNetwork_ManagedServices_ServiceNetworkEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_ServiceNetworkEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_type` after provisioning.\n"]
    pub fn vpc_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_StsAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sts_policy_document: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_StsAccess {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sts_policy_document`.\n"]
    pub fn set_sts_policy_document(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sts_policy_document = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_StsAccess {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_StsAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_StsAccess {}

impl BuildDataOdbNetwork_ManagedServices_StsAccess {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_StsAccess {
        DataOdbNetwork_ManagedServices_StsAccess {
            domain_name: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            status: core::default::Default::default(),
            sts_policy_document: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_StsAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_StsAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbNetwork_ManagedServices_StsAccessRef {
        DataOdbNetwork_ManagedServices_StsAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_StsAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `sts_policy_document` after provisioning.\n"]
    pub fn sts_policy_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sts_policy_document", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices_ZeroTlAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_ManagedServices_ZeroTlAccess {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices_ZeroTlAccess {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices_ZeroTlAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices_ZeroTlAccess {}

impl BuildDataOdbNetwork_ManagedServices_ZeroTlAccess {
    pub fn build(self) -> DataOdbNetwork_ManagedServices_ZeroTlAccess {
        DataOdbNetwork_ManagedServices_ZeroTlAccess {
            cidr: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServices_ZeroTlAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServices_ZeroTlAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbNetwork_ManagedServices_ZeroTlAccessRef {
        DataOdbNetwork_ManagedServices_ZeroTlAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServices_ZeroTlAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_ManagedServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_s3_restore_sources_access: std
    ::option
    ::Option<terrars::ListField<DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_access: std::option::Option<terrars::ListField<DataOdbNetwork_ManagedServices_KmsAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_s3_backup_access: std
    ::option
    ::Option<terrars::ListField<DataOdbNetwork_ManagedServices_ManagedS3BackupAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_service_ipv4_cidrs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_gateway_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access: std::option::Option<terrars::ListField<DataOdbNetwork_ManagedServices_S3Access>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_endpoint: std
    ::option
    ::Option<terrars::ListField<DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sts_access: std::option::Option<terrars::ListField<DataOdbNetwork_ManagedServices_StsAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_tl_access: std::option::Option<terrars::ListField<DataOdbNetwork_ManagedServices_ZeroTlAccess>>,
}

impl DataOdbNetwork_ManagedServices {
    #[doc = "Set the field `cross_region_s3_restore_sources_access`.\n"]
    pub fn set_cross_region_s3_restore_sources_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccess>>,
    ) -> Self {
        self.cross_region_s3_restore_sources_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_access`.\n"]
    pub fn set_kms_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbNetwork_ManagedServices_KmsAccess>>,
    ) -> Self {
        self.kms_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_s3_backup_access`.\n"]
    pub fn set_managed_s3_backup_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbNetwork_ManagedServices_ManagedS3BackupAccess>>,
    ) -> Self {
        self.managed_s3_backup_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_service_ipv4_cidrs`.\n"]
    pub fn set_managed_service_ipv4_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.managed_service_ipv4_cidrs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_gateway_arn`.\n"]
    pub fn set_resource_gateway_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_gateway_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access`.\n"]
    pub fn set_s3_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbNetwork_ManagedServices_S3Access>>,
    ) -> Self {
        self.s3_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_arn`.\n"]
    pub fn set_service_network_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_network_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_endpoint`.\n"]
    pub fn set_service_network_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbNetwork_ManagedServices_ServiceNetworkEndpoint>>,
    ) -> Self {
        self.service_network_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sts_access`.\n"]
    pub fn set_sts_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbNetwork_ManagedServices_StsAccess>>,
    ) -> Self {
        self.sts_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zero_tl_access`.\n"]
    pub fn set_zero_tl_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbNetwork_ManagedServices_ZeroTlAccess>>,
    ) -> Self {
        self.zero_tl_access = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_ManagedServices {
    type O = terrars::BlockAssignable<DataOdbNetwork_ManagedServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_ManagedServices {}

impl BuildDataOdbNetwork_ManagedServices {
    pub fn build(self) -> DataOdbNetwork_ManagedServices {
        DataOdbNetwork_ManagedServices {
            cross_region_s3_restore_sources_access: core::default::Default::default(),
            kms_access: core::default::Default::default(),
            managed_s3_backup_access: core::default::Default::default(),
            managed_service_ipv4_cidrs: core::default::Default::default(),
            resource_gateway_arn: core::default::Default::default(),
            s3_access: core::default::Default::default(),
            service_network_arn: core::default::Default::default(),
            service_network_endpoint: core::default::Default::default(),
            sts_access: core::default::Default::default(),
            zero_tl_access: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_ManagedServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_ManagedServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbNetwork_ManagedServicesRef {
        DataOdbNetwork_ManagedServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_ManagedServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_region_s3_restore_sources_access` after provisioning.\n"]
    pub fn cross_region_s3_restore_sources_access(
        &self,
    ) -> terrars::ListRef<DataOdbNetwork_ManagedServices_CrossRegionS3RestoreSourcesAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_s3_restore_sources_access", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_access` after provisioning.\n"]
    pub fn kms_access(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServices_KmsAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_access", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_s3_backup_access` after provisioning.\n"]
    pub fn managed_s3_backup_access(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServices_ManagedS3BackupAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_s3_backup_access", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_service_ipv4_cidrs` after provisioning.\n"]
    pub fn managed_service_ipv4_cidrs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_service_ipv4_cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_gateway_arn` after provisioning.\n"]
    pub fn resource_gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_gateway_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access` after provisioning.\n"]
    pub fn s3_access(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServices_S3AccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_access", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_arn` after provisioning.\n"]
    pub fn service_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_endpoint` after provisioning.\n"]
    pub fn service_network_endpoint(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServices_ServiceNetworkEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_network_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `sts_access` after provisioning.\n"]
    pub fn sts_access(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServices_StsAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sts_access", self.base))
    }

    #[doc = "Get a reference to the value of field `zero_tl_access` after provisioning.\n"]
    pub fn zero_tl_access(&self) -> terrars::ListRef<DataOdbNetwork_ManagedServices_ZeroTlAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zero_tl_access", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetwork_OciDnsForwardingConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_dns_listener_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetwork_OciDnsForwardingConfigs {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_dns_listener_ip`.\n"]
    pub fn set_oci_dns_listener_ip(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oci_dns_listener_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetwork_OciDnsForwardingConfigs {
    type O = terrars::BlockAssignable<DataOdbNetwork_OciDnsForwardingConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetwork_OciDnsForwardingConfigs {}

impl BuildDataOdbNetwork_OciDnsForwardingConfigs {
    pub fn build(self) -> DataOdbNetwork_OciDnsForwardingConfigs {
        DataOdbNetwork_OciDnsForwardingConfigs {
            domain_name: core::default::Default::default(),
            oci_dns_listener_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetwork_OciDnsForwardingConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetwork_OciDnsForwardingConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbNetwork_OciDnsForwardingConfigsRef {
        DataOdbNetwork_OciDnsForwardingConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetwork_OciDnsForwardingConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_dns_listener_ip` after provisioning.\n"]
    pub fn oci_dns_listener_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_dns_listener_ip", self.base))
    }
}
