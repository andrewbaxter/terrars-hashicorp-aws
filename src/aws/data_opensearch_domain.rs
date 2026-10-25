#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOpensearchDomainData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataOpensearchDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOpensearchDomainData_>,
}

#[derive(Clone)]
pub struct DataOpensearchDomain(std::rc::Rc<DataOpensearchDomain_>);

impl DataOpensearchDomain {
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

    #[doc = "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options` after provisioning.\n"]
    pub fn advanced_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.advanced_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(&self) -> terrars::ListRef<DataOpensearchDomain_AdvancedSecurityOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<DataOpensearchDomain_AutoTuneOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<DataOpensearchDomain_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<DataOpensearchDomain_CognitoOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint` after provisioning.\n"]
    pub fn dashboard_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint_v2` after provisioning.\n"]
    pub fn dashboard_endpoint_v2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_strategy_options` after provisioning.\n"]
    pub fn deployment_strategy_options(&self) -> terrars::ListRef<DataOpensearchDomain_DeploymentStrategyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_strategy_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_v2_hosted_zone_id` after provisioning.\n"]
    pub fn domain_endpoint_v2_hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_v2_hosted_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<DataOpensearchDomain_EbsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> terrars::ListRef<DataOpensearchDomain_EncryptionAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_v2` after provisioning.\n"]
    pub fn endpoint_v2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(&self) -> terrars::ListRef<DataOpensearchDomain_IdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> terrars::SetRef<DataOpensearchDomain_LogPublishingOptionsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<DataOpensearchDomain_NodeToNodeEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(&self) -> terrars::ListRef<DataOpensearchDomain_OffPeakWindowOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.off_peak_window_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processing` after provisioning.\n"]
    pub fn processing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<DataOpensearchDomain_SnapshotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(&self) -> terrars::ListRef<DataOpensearchDomain_SoftwareUpdateOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_update_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<DataOpensearchDomain_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

impl terrars::Referable for DataOpensearchDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOpensearchDomain { }

impl terrars::ToListMappable for DataOpensearchDomain {
    type O = terrars::ListRef<DataOpensearchDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOpensearchDomain_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_opensearch_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildDataOpensearchDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOpensearchDomain {
        let out = DataOpensearchDomain(std::rc::Rc::new(DataOpensearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOpensearchDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOpensearchDomainRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options` after provisioning.\n"]
    pub fn advanced_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.advanced_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(&self) -> terrars::ListRef<DataOpensearchDomain_AdvancedSecurityOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<DataOpensearchDomain_AutoTuneOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<DataOpensearchDomain_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<DataOpensearchDomain_CognitoOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint` after provisioning.\n"]
    pub fn dashboard_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint_v2` after provisioning.\n"]
    pub fn dashboard_endpoint_v2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_strategy_options` after provisioning.\n"]
    pub fn deployment_strategy_options(&self) -> terrars::ListRef<DataOpensearchDomain_DeploymentStrategyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_strategy_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_v2_hosted_zone_id` after provisioning.\n"]
    pub fn domain_endpoint_v2_hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_v2_hosted_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<DataOpensearchDomain_EbsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> terrars::ListRef<DataOpensearchDomain_EncryptionAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_v2` after provisioning.\n"]
    pub fn endpoint_v2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(&self) -> terrars::ListRef<DataOpensearchDomain_IdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> terrars::SetRef<DataOpensearchDomain_LogPublishingOptionsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<DataOpensearchDomain_NodeToNodeEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(&self) -> terrars::ListRef<DataOpensearchDomain_OffPeakWindowOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.off_peak_window_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processing` after provisioning.\n"]
    pub fn processing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<DataOpensearchDomain_SnapshotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(&self) -> terrars::ListRef<DataOpensearchDomain_SoftwareUpdateOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_update_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<DataOpensearchDomain_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwks_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwks_url`.\n"]
    pub fn set_jwks_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwks_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles_key`.\n"]
    pub fn set_roles_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.roles_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_key`.\n"]
    pub fn set_subject_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {}

impl BuildDataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
    pub fn build(self) -> DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
        DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
            enabled: core::default::Default::default(),
            jwks_url: core::default::Default::default(),
            public_key: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            subject_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
        DataOpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_url` after provisioning.\n"]
    pub fn jwks_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_url", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `roles_key` after provisioning.\n"]
    pub fn roles_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.roles_key", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_key` after provisioning.\n"]
    pub fn subject_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_AdvancedSecurityOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonymous_auth_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_user_database_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_options: std::option::Option<terrars::ListField<DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions>>,
}

impl DataOpensearchDomain_AdvancedSecurityOptions {
    #[doc = "Set the field `anonymous_auth_enabled`.\n"]
    pub fn set_anonymous_auth_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.anonymous_auth_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_user_database_enabled`.\n"]
    pub fn set_internal_user_database_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_user_database_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_options`.\n"]
    pub fn set_jwt_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomain_AdvancedSecurityOptions_JwtOptions>>,
    ) -> Self {
        self.jwt_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_AdvancedSecurityOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_AdvancedSecurityOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_AdvancedSecurityOptions {}

impl BuildDataOpensearchDomain_AdvancedSecurityOptions {
    pub fn build(self) -> DataOpensearchDomain_AdvancedSecurityOptions {
        DataOpensearchDomain_AdvancedSecurityOptions {
            anonymous_auth_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            internal_user_database_enabled: core::default::Default::default(),
            jwt_options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_AdvancedSecurityOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_AdvancedSecurityOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_AdvancedSecurityOptionsRef {
        DataOpensearchDomain_AdvancedSecurityOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_AdvancedSecurityOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `anonymous_auth_enabled` after provisioning.\n"]
    pub fn anonymous_auth_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anonymous_auth_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_user_database_enabled` after provisioning.\n"]
    pub fn internal_user_database_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_user_database_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_options` after provisioning.\n"]
    pub fn jwt_options(&self) -> terrars::ListRef<DataOpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    type O = terrars::BlockAssignable<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {}

impl BuildDataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    pub fn build(self) -> DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
        DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
        DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_expression_for_recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::ListField<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    #[doc = "Set the field `cron_expression_for_recurrence`.\n"]
    pub fn set_cron_expression_for_recurrence(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cron_expression_for_recurrence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
    ) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_at`.\n"]
    pub fn set_start_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    type O = terrars::BlockAssignable<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {}

impl BuildDataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    pub fn build(self) -> DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
        DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
            cron_expression_for_recurrence: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
        DataOpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_expression_for_recurrence` after provisioning.\n"]
    pub fn cron_expression_for_recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_expression_for_recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::ListRef<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_at` after provisioning.\n"]
    pub fn start_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_AutoTuneOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule: std
    ::option
    ::Option<terrars::SetField<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_on_disable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_off_peak_window: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomain_AutoTuneOptions {
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataOpensearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
    ) -> Self {
        self.maintenance_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollback_on_disable`.\n"]
    pub fn set_rollback_on_disable(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rollback_on_disable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_off_peak_window`.\n"]
    pub fn set_use_off_peak_window(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_off_peak_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_AutoTuneOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_AutoTuneOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_AutoTuneOptions {}

impl BuildDataOpensearchDomain_AutoTuneOptions {
    pub fn build(self) -> DataOpensearchDomain_AutoTuneOptions {
        DataOpensearchDomain_AutoTuneOptions {
            desired_state: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            rollback_on_disable: core::default::Default::default(),
            use_off_peak_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_AutoTuneOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_AutoTuneOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_AutoTuneOptionsRef {
        DataOpensearchDomain_AutoTuneOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_AutoTuneOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\n"]
    pub fn maintenance_schedule(&self) -> terrars::SetRef<DataOpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback_on_disable` after provisioning.\n"]
    pub fn rollback_on_disable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback_on_disable", self.base))
    }

    #[doc = "Get a reference to the value of field `use_off_peak_window` after provisioning.\n"]
    pub fn use_off_peak_window(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_off_peak_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_ClusterConfig_ColdStorageOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomain_ClusterConfig_ColdStorageOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_ClusterConfig_ColdStorageOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_ClusterConfig_ColdStorageOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_ClusterConfig_ColdStorageOptions {}

impl BuildDataOpensearchDomain_ClusterConfig_ColdStorageOptions {
    pub fn build(self) -> DataOpensearchDomain_ClusterConfig_ColdStorageOptions {
        DataOpensearchDomain_ClusterConfig_ColdStorageOptions { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
        DataOpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    type O = terrars::BlockAssignable<DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {}

impl BuildDataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    pub fn build(self) -> DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
        DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
            count: core::default::Default::default(),
            enabled: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
        DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_ClusterConfig_NodeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<terrars::ListField<DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_ClusterConfig_NodeOptions {
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>>,
    ) -> Self {
        self.node_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_ClusterConfig_NodeOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_ClusterConfig_NodeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_ClusterConfig_NodeOptions {}

impl BuildDataOpensearchDomain_ClusterConfig_NodeOptions {
    pub fn build(self) -> DataOpensearchDomain_ClusterConfig_NodeOptions {
        DataOpensearchDomain_ClusterConfig_NodeOptions {
            node_config: core::default::Default::default(),
            node_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_ClusterConfig_NodeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_ClusterConfig_NodeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_ClusterConfig_NodeOptionsRef {
        DataOpensearchDomain_ClusterConfig_NodeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_ClusterConfig_NodeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<DataOpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_zone_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    type O = terrars::BlockAssignable<DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {}

impl BuildDataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    pub fn build(self) -> DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
        DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
            availability_zone_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
        DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_count` after provisioning.\n"]
    pub fn availability_zone_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_ClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_options: std::option::Option<terrars::ListField<DataOpensearchDomain_ClusterConfig_ColdStorageOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_master_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_master_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_master_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_az_with_standby_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_options: std::option::Option<terrars::ListField<DataOpensearchDomain_ClusterConfig_NodeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_config: std
    ::option
    ::Option<terrars::ListField<DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomain_ClusterConfig {
    #[doc = "Set the field `cold_storage_options`.\n"]
    pub fn set_cold_storage_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomain_ClusterConfig_ColdStorageOptions>>,
    ) -> Self {
        self.cold_storage_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_master_count`.\n"]
    pub fn set_dedicated_master_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dedicated_master_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_master_enabled`.\n"]
    pub fn set_dedicated_master_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dedicated_master_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_master_type`.\n"]
    pub fn set_dedicated_master_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dedicated_master_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_az_with_standby_enabled`.\n"]
    pub fn set_multi_az_with_standby_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.multi_az_with_standby_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_options`.\n"]
    pub fn set_node_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomain_ClusterConfig_NodeOptions>>,
    ) -> Self {
        self.node_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warm_count`.\n"]
    pub fn set_warm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.warm_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warm_enabled`.\n"]
    pub fn set_warm_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.warm_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warm_type`.\n"]
    pub fn set_warm_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.warm_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_awareness_config`.\n"]
    pub fn set_zone_awareness_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
    ) -> Self {
        self.zone_awareness_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_awareness_enabled`.\n"]
    pub fn set_zone_awareness_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.zone_awareness_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_ClusterConfig {
    type O = terrars::BlockAssignable<DataOpensearchDomain_ClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_ClusterConfig {}

impl BuildDataOpensearchDomain_ClusterConfig {
    pub fn build(self) -> DataOpensearchDomain_ClusterConfig {
        DataOpensearchDomain_ClusterConfig {
            cold_storage_options: core::default::Default::default(),
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            multi_az_with_standby_enabled: core::default::Default::default(),
            node_options: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_ClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_ClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_ClusterConfigRef {
        DataOpensearchDomain_ClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_ClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cold_storage_options` after provisioning.\n"]
    pub fn cold_storage_options(&self) -> terrars::ListRef<DataOpensearchDomain_ClusterConfig_ColdStorageOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cold_storage_options", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_master_count` after provisioning.\n"]
    pub fn dedicated_master_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_master_count", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_master_enabled` after provisioning.\n"]
    pub fn dedicated_master_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_master_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated_master_type` after provisioning.\n"]
    pub fn dedicated_master_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_master_type", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_az_with_standby_enabled` after provisioning.\n"]
    pub fn multi_az_with_standby_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az_with_standby_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `node_options` after provisioning.\n"]
    pub fn node_options(&self) -> terrars::ListRef<DataOpensearchDomain_ClusterConfig_NodeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_options", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_count` after provisioning.\n"]
    pub fn warm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_count", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_enabled` after provisioning.\n"]
    pub fn warm_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_type` after provisioning.\n"]
    pub fn warm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warm_type", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(&self) -> terrars::ListRef<DataOpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_awareness_config", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_enabled` after provisioning.\n"]
    pub fn zone_awareness_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_awareness_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_CognitoOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_CognitoOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_pool_id`.\n"]
    pub fn set_identity_pool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_id`.\n"]
    pub fn set_user_pool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_pool_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_CognitoOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_CognitoOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_CognitoOptions {}

impl BuildDataOpensearchDomain_CognitoOptions {
    pub fn build(self) -> DataOpensearchDomain_CognitoOptions {
        DataOpensearchDomain_CognitoOptions {
            enabled: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_pool_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_CognitoOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_CognitoOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_CognitoOptionsRef {
        DataOpensearchDomain_CognitoOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_CognitoOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_DeploymentStrategyOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_DeploymentStrategyOptions {
    #[doc = "Set the field `deployment_strategy`.\n"]
    pub fn set_deployment_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deployment_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_DeploymentStrategyOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_DeploymentStrategyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_DeploymentStrategyOptions {}

impl BuildDataOpensearchDomain_DeploymentStrategyOptions {
    pub fn build(self) -> DataOpensearchDomain_DeploymentStrategyOptions {
        DataOpensearchDomain_DeploymentStrategyOptions { deployment_strategy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_DeploymentStrategyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_DeploymentStrategyOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_DeploymentStrategyOptionsRef {
        DataOpensearchDomain_DeploymentStrategyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_DeploymentStrategyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_strategy` after provisioning.\n"]
    pub fn deployment_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_EbsOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_EbsOptions {
    #[doc = "Set the field `ebs_enabled`.\n"]
    pub fn set_ebs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ebs_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_EbsOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_EbsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_EbsOptions {}

impl BuildDataOpensearchDomain_EbsOptions {
    pub fn build(self) -> DataOpensearchDomain_EbsOptions {
        DataOpensearchDomain_EbsOptions {
            ebs_enabled: core::default::Default::default(),
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_EbsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_EbsOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_EbsOptionsRef {
        DataOpensearchDomain_EbsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_EbsOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_enabled` after provisioning.\n"]
    pub fn ebs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_EncryptionAtRest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_EncryptionAtRest {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_EncryptionAtRest {
    type O = terrars::BlockAssignable<DataOpensearchDomain_EncryptionAtRest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_EncryptionAtRest {}

impl BuildDataOpensearchDomain_EncryptionAtRest {
    pub fn build(self) -> DataOpensearchDomain_EncryptionAtRest {
        DataOpensearchDomain_EncryptionAtRest {
            enabled: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_EncryptionAtRestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_EncryptionAtRestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_EncryptionAtRestRef {
        DataOpensearchDomain_EncryptionAtRestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_EncryptionAtRestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_IdentityCenterOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_api_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_IdentityCenterOptions {
    #[doc = "Set the field `enabled_api_access`.\n"]
    pub fn set_enabled_api_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled_api_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_center_instance_arn`.\n"]
    pub fn set_identity_center_instance_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_center_instance_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles_key`.\n"]
    pub fn set_roles_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.roles_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_key`.\n"]
    pub fn set_subject_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_IdentityCenterOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_IdentityCenterOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_IdentityCenterOptions {}

impl BuildDataOpensearchDomain_IdentityCenterOptions {
    pub fn build(self) -> DataOpensearchDomain_IdentityCenterOptions {
        DataOpensearchDomain_IdentityCenterOptions {
            enabled_api_access: core::default::Default::default(),
            identity_center_instance_arn: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            subject_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_IdentityCenterOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_IdentityCenterOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_IdentityCenterOptionsRef {
        DataOpensearchDomain_IdentityCenterOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_IdentityCenterOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_api_access` after provisioning.\n"]
    pub fn enabled_api_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_api_access", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `roles_key` after provisioning.\n"]
    pub fn roles_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.roles_key", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_key` after provisioning.\n"]
    pub fn subject_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_LogPublishingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_LogPublishingOptions {
    #[doc = "Set the field `cloudwatch_log_group_arn`.\n"]
    pub fn set_cloudwatch_log_group_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloudwatch_log_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\n"]
    pub fn set_log_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_LogPublishingOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_LogPublishingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_LogPublishingOptions {}

impl BuildDataOpensearchDomain_LogPublishingOptions {
    pub fn build(self) -> DataOpensearchDomain_LogPublishingOptions {
        DataOpensearchDomain_LogPublishingOptions {
            cloudwatch_log_group_arn: core::default::Default::default(),
            enabled: core::default::Default::default(),
            log_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_LogPublishingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_LogPublishingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_LogPublishingOptionsRef {
        DataOpensearchDomain_LogPublishingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_LogPublishingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_NodeToNodeEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomain_NodeToNodeEncryption {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_NodeToNodeEncryption {
    type O = terrars::BlockAssignable<DataOpensearchDomain_NodeToNodeEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_NodeToNodeEncryption {}

impl BuildDataOpensearchDomain_NodeToNodeEncryption {
    pub fn build(self) -> DataOpensearchDomain_NodeToNodeEncryption {
        DataOpensearchDomain_NodeToNodeEncryption { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_NodeToNodeEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_NodeToNodeEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_NodeToNodeEncryptionRef {
        DataOpensearchDomain_NodeToNodeEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_NodeToNodeEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    type O = terrars::BlockAssignable<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {}

impl BuildDataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    pub fn build(self) -> DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
        DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
        DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_start_time: std
    ::option
    ::Option<terrars::ListField<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>>,
}

impl DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>,
                    >,
    ) -> Self {
        self.window_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    type O = terrars::BlockAssignable<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {}

impl BuildDataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    pub fn build(self) -> DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
        DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
            window_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
        DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(
        &self,
    ) -> terrars::ListRef<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_OffPeakWindowOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    off_peak_window: std::option::Option<terrars::ListField<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow>>,
}

impl DataOpensearchDomain_OffPeakWindowOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `off_peak_window`.\n"]
    pub fn set_off_peak_window(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindow>>,
    ) -> Self {
        self.off_peak_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_OffPeakWindowOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_OffPeakWindowOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_OffPeakWindowOptions {}

impl BuildDataOpensearchDomain_OffPeakWindowOptions {
    pub fn build(self) -> DataOpensearchDomain_OffPeakWindowOptions {
        DataOpensearchDomain_OffPeakWindowOptions {
            enabled: core::default::Default::default(),
            off_peak_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_OffPeakWindowOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_OffPeakWindowOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_OffPeakWindowOptionsRef {
        DataOpensearchDomain_OffPeakWindowOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_OffPeakWindowOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `off_peak_window` after provisioning.\n"]
    pub fn off_peak_window(&self) -> terrars::ListRef<DataOpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.off_peak_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_SnapshotOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_snapshot_start_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomain_SnapshotOptions {
    #[doc = "Set the field `automated_snapshot_start_hour`.\n"]
    pub fn set_automated_snapshot_start_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.automated_snapshot_start_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_SnapshotOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_SnapshotOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_SnapshotOptions {}

impl BuildDataOpensearchDomain_SnapshotOptions {
    pub fn build(self) -> DataOpensearchDomain_SnapshotOptions {
        DataOpensearchDomain_SnapshotOptions { automated_snapshot_start_hour: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_SnapshotOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_SnapshotOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_SnapshotOptionsRef {
        DataOpensearchDomain_SnapshotOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_SnapshotOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_start_hour` after provisioning.\n"]
    pub fn automated_snapshot_start_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automated_snapshot_start_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_SoftwareUpdateOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_software_update_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomain_SoftwareUpdateOptions {
    #[doc = "Set the field `auto_software_update_enabled`.\n"]
    pub fn set_auto_software_update_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_software_update_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_SoftwareUpdateOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_SoftwareUpdateOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_SoftwareUpdateOptions {}

impl BuildDataOpensearchDomain_SoftwareUpdateOptions {
    pub fn build(self) -> DataOpensearchDomain_SoftwareUpdateOptions {
        DataOpensearchDomain_SoftwareUpdateOptions {
            auto_software_update_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_SoftwareUpdateOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_SoftwareUpdateOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_SoftwareUpdateOptionsRef {
        DataOpensearchDomain_SoftwareUpdateOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_SoftwareUpdateOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_software_update_enabled` after provisioning.\n"]
    pub fn auto_software_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_software_update_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomain_VpcOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomain_VpcOptions {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.availability_zones = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomain_VpcOptions {
    type O = terrars::BlockAssignable<DataOpensearchDomain_VpcOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomain_VpcOptions {}

impl BuildDataOpensearchDomain_VpcOptions {
    pub fn build(self) -> DataOpensearchDomain_VpcOptions {
        DataOpensearchDomain_VpcOptions {
            availability_zones: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOpensearchDomain_VpcOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomain_VpcOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomain_VpcOptionsRef {
        DataOpensearchDomain_VpcOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomain_VpcOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
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
