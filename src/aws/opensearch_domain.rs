#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchDomainData_ {
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
    access_policies: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_security_options: std::option::Option<std::vec::Vec<OpensearchDomain_AdvancedSecurityOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aiml_options: std::option::Option<std::vec::Vec<OpensearchDomain_AimlOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_tune_options: std::option::Option<std::vec::Vec<OpensearchDomain_AutoTuneOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_config: std::option::Option<std::vec::Vec<OpensearchDomain_ClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_options: std::option::Option<std::vec::Vec<OpensearchDomain_CognitoOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_strategy_options: std::option::Option<std::vec::Vec<OpensearchDomain_DeploymentStrategyOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_endpoint_options: std::option::Option<std::vec::Vec<OpensearchDomain_DomainEndpointOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_options: std::option::Option<std::vec::Vec<OpensearchDomain_EbsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypt_at_rest: std::option::Option<std::vec::Vec<OpensearchDomain_EncryptAtRest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_options: std::option::Option<std::vec::Vec<OpensearchDomain_IdentityCenterOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_publishing_options: std::option::Option<std::vec::Vec<OpensearchDomain_LogPublishingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_to_node_encryption: std::option::Option<std::vec::Vec<OpensearchDomain_NodeToNodeEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    off_peak_window_options: std::option::Option<std::vec::Vec<OpensearchDomain_OffPeakWindowOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_options: std::option::Option<std::vec::Vec<OpensearchDomain_SnapshotOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_update_options: std::option::Option<std::vec::Vec<OpensearchDomain_SoftwareUpdateOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OpensearchDomain_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_options: std::option::Option<std::vec::Vec<OpensearchDomain_VpcOptions>>,
    dynamic: OpensearchDomainDynamic,
}

struct OpensearchDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpensearchDomainData_>,
}

#[derive(Clone)]
pub struct OpensearchDomain(std::rc::Rc<OpensearchDomain_>);

impl OpensearchDomain {
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

    #[doc = "Set the field `access_policies`.\n"]
    pub fn set_access_policies(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_options`.\n"]
    pub fn set_advanced_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().advanced_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_security_options`.\n"]
    pub fn set_advanced_security_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_AdvancedSecurityOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_security_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_security_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aiml_options`.\n"]
    pub fn set_aiml_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_AimlOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aiml_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aiml_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_tune_options`.\n"]
    pub fn set_auto_tune_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_AutoTuneOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_tune_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_tune_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_ClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cognito_options`.\n"]
    pub fn set_cognito_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_CognitoOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cognito_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cognito_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_strategy_options`.\n"]
    pub fn set_deployment_strategy_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_DeploymentStrategyOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_strategy_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_strategy_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `domain_endpoint_options`.\n"]
    pub fn set_domain_endpoint_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_DomainEndpointOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_endpoint_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_endpoint_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_options`.\n"]
    pub fn set_ebs_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_EbsOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encrypt_at_rest`.\n"]
    pub fn set_encrypt_at_rest(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_EncryptAtRest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encrypt_at_rest = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encrypt_at_rest = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_center_options`.\n"]
    pub fn set_identity_center_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_IdentityCenterOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_center_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_center_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_publishing_options`.\n"]
    pub fn set_log_publishing_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_LogPublishingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_publishing_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_publishing_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_to_node_encryption`.\n"]
    pub fn set_node_to_node_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_NodeToNodeEncryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_to_node_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_to_node_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `off_peak_window_options`.\n"]
    pub fn set_off_peak_window_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_OffPeakWindowOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().off_peak_window_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.off_peak_window_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snapshot_options`.\n"]
    pub fn set_snapshot_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_SnapshotOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_update_options`.\n"]
    pub fn set_software_update_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_SoftwareUpdateOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().software_update_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.software_update_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OpensearchDomain_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_VpcOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_options = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint` after provisioning.\n"]
    pub fn dashboard_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint_v2` after provisioning.\n"]
    pub fn dashboard_endpoint_v2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint_v2", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(&self) -> terrars::ListRef<OpensearchDomain_AdvancedSecurityOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aiml_options` after provisioning.\n"]
    pub fn aiml_options(&self) -> terrars::ListRef<OpensearchDomain_AimlOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aiml_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<OpensearchDomain_AutoTuneOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<OpensearchDomain_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<OpensearchDomain_CognitoOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_strategy_options` after provisioning.\n"]
    pub fn deployment_strategy_options(&self) -> terrars::ListRef<OpensearchDomain_DeploymentStrategyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_strategy_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> terrars::ListRef<OpensearchDomain_DomainEndpointOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<OpensearchDomain_EbsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> terrars::ListRef<OpensearchDomain_EncryptAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encrypt_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(&self) -> terrars::ListRef<OpensearchDomain_IdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<OpensearchDomain_NodeToNodeEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(&self) -> terrars::ListRef<OpensearchDomain_OffPeakWindowOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.off_peak_window_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<OpensearchDomain_SnapshotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(&self) -> terrars::ListRef<OpensearchDomain_SoftwareUpdateOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_update_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchDomain_TimeoutsRef {
        OpensearchDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<OpensearchDomain_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

impl terrars::Referable for OpensearchDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpensearchDomain { }

impl terrars::ToListMappable for OpensearchDomain {
    type O = terrars::ListRef<OpensearchDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpensearchDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opensearch_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> OpensearchDomain {
        let out = OpensearchDomain(std::rc::Rc::new(OpensearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpensearchDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_policies: core::default::Default::default(),
                advanced_options: core::default::Default::default(),
                domain_name: self.domain_name,
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_security_options: core::default::Default::default(),
                aiml_options: core::default::Default::default(),
                auto_tune_options: core::default::Default::default(),
                cluster_config: core::default::Default::default(),
                cognito_options: core::default::Default::default(),
                deployment_strategy_options: core::default::Default::default(),
                domain_endpoint_options: core::default::Default::default(),
                ebs_options: core::default::Default::default(),
                encrypt_at_rest: core::default::Default::default(),
                identity_center_options: core::default::Default::default(),
                log_publishing_options: core::default::Default::default(),
                node_to_node_encryption: core::default::Default::default(),
                off_peak_window_options: core::default::Default::default(),
                snapshot_options: core::default::Default::default(),
                software_update_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchDomainRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options` after provisioning.\n"]
    pub fn advanced_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.advanced_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint` after provisioning.\n"]
    pub fn dashboard_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint_v2` after provisioning.\n"]
    pub fn dashboard_endpoint_v2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint_v2", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(&self) -> terrars::ListRef<OpensearchDomain_AdvancedSecurityOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aiml_options` after provisioning.\n"]
    pub fn aiml_options(&self) -> terrars::ListRef<OpensearchDomain_AimlOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aiml_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<OpensearchDomain_AutoTuneOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<OpensearchDomain_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<OpensearchDomain_CognitoOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_strategy_options` after provisioning.\n"]
    pub fn deployment_strategy_options(&self) -> terrars::ListRef<OpensearchDomain_DeploymentStrategyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_strategy_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> terrars::ListRef<OpensearchDomain_DomainEndpointOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<OpensearchDomain_EbsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> terrars::ListRef<OpensearchDomain_EncryptAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encrypt_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(&self) -> terrars::ListRef<OpensearchDomain_IdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<OpensearchDomain_NodeToNodeEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(&self) -> terrars::ListRef<OpensearchDomain_OffPeakWindowOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.off_peak_window_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<OpensearchDomain_SnapshotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(&self) -> terrars::ListRef<OpensearchDomain_SoftwareUpdateOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_update_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchDomain_TimeoutsRef {
        OpensearchDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<OpensearchDomain_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OpensearchDomainDynamic {
    advanced_security_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_AdvancedSecurityOptions>>,
    aiml_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_AimlOptions>>,
    auto_tune_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_AutoTuneOptions>>,
    cluster_config: std::option::Option<terrars::DynamicBlock<OpensearchDomain_ClusterConfig>>,
    cognito_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_CognitoOptions>>,
    deployment_strategy_options: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_DeploymentStrategyOptions>>,
    domain_endpoint_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_DomainEndpointOptions>>,
    ebs_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_EbsOptions>>,
    encrypt_at_rest: std::option::Option<terrars::DynamicBlock<OpensearchDomain_EncryptAtRest>>,
    identity_center_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_IdentityCenterOptions>>,
    log_publishing_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_LogPublishingOptions>>,
    node_to_node_encryption: std::option::Option<terrars::DynamicBlock<OpensearchDomain_NodeToNodeEncryption>>,
    off_peak_window_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_OffPeakWindowOptions>>,
    snapshot_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_SnapshotOptions>>,
    software_update_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_SoftwareUpdateOptions>>,
    vpc_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_VpcOptions>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_AdvancedSecurityOptions_JwtOptions {
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

impl OpensearchDomain_AdvancedSecurityOptions_JwtOptions {
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

impl terrars::ToListMappable for OpensearchDomain_AdvancedSecurityOptions_JwtOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_AdvancedSecurityOptions_JwtOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AdvancedSecurityOptions_JwtOptions {}

impl BuildOpensearchDomain_AdvancedSecurityOptions_JwtOptions {
    pub fn build(self) -> OpensearchDomain_AdvancedSecurityOptions_JwtOptions {
        OpensearchDomain_AdvancedSecurityOptions_JwtOptions {
            enabled: core::default::Default::default(),
            jwks_url: core::default::Default::default(),
            public_key: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            subject_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
        OpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef {
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
pub struct OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_password: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    #[doc = "Set the field `master_user_arn`.\n"]
    pub fn set_master_user_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.master_user_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_user_name`.\n"]
    pub fn set_master_user_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.master_user_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_user_password`.\n"]
    pub fn set_master_user_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_user_password = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {}

impl BuildOpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    pub fn build(self) -> OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {
        OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions {
            master_user_arn: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            master_user_password: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
        OpensearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `master_user_arn` after provisioning.\n"]
    pub fn master_user_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_user_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `master_user_name` after provisioning.\n"]
    pub fn master_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_user_name", self.base))
    }

    #[doc = "Get a reference to the value of field `master_user_password` after provisioning.\n"]
    pub fn master_user_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_user_password", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchDomain_AdvancedSecurityOptionsDynamic {
    jwt_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_AdvancedSecurityOptions_JwtOptions>>,
    master_user_options: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_AdvancedSecurityOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonymous_auth_enabled: std::option::Option<terrars::PrimField<bool>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_user_database_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_options: std::option::Option<std::vec::Vec<OpensearchDomain_AdvancedSecurityOptions_JwtOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_options: std::option::Option<std::vec::Vec<OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions>>,
    dynamic: OpensearchDomain_AdvancedSecurityOptionsDynamic,
}

impl OpensearchDomain_AdvancedSecurityOptions {
    #[doc = "Set the field `anonymous_auth_enabled`.\n"]
    pub fn set_anonymous_auth_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.anonymous_auth_enabled = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_AdvancedSecurityOptions_JwtOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jwt_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_user_options`.\n"]
    pub fn set_master_user_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpensearchDomain_AdvancedSecurityOptions_MasterUserOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.master_user_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.master_user_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AdvancedSecurityOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_AdvancedSecurityOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AdvancedSecurityOptions {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildOpensearchDomain_AdvancedSecurityOptions {
    pub fn build(self) -> OpensearchDomain_AdvancedSecurityOptions {
        OpensearchDomain_AdvancedSecurityOptions {
            anonymous_auth_enabled: core::default::Default::default(),
            enabled: self.enabled,
            internal_user_database_enabled: core::default::Default::default(),
            jwt_options: core::default::Default::default(),
            master_user_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AdvancedSecurityOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AdvancedSecurityOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_AdvancedSecurityOptionsRef {
        OpensearchDomain_AdvancedSecurityOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AdvancedSecurityOptionsRef {
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
    pub fn jwt_options(&self) -> terrars::ListRef<OpensearchDomain_AdvancedSecurityOptions_JwtOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_options", self.base))
    }

    #[doc = "Get a reference to the value of field `master_user_options` after provisioning.\n"]
    pub fn master_user_options(&self) -> terrars::ListRef<OpensearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_user_options", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.desired_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {}

impl BuildOpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {
    pub fn build(self) -> OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {
        OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions {
            desired_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptionsRef {
        OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_AimlOptions_S3VectorsEngine {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl OpensearchDomain_AimlOptions_S3VectorsEngine {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AimlOptions_S3VectorsEngine {
    type O = terrars::BlockAssignable<OpensearchDomain_AimlOptions_S3VectorsEngine>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AimlOptions_S3VectorsEngine {}

impl BuildOpensearchDomain_AimlOptions_S3VectorsEngine {
    pub fn build(self) -> OpensearchDomain_AimlOptions_S3VectorsEngine {
        OpensearchDomain_AimlOptions_S3VectorsEngine { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AimlOptions_S3VectorsEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AimlOptions_S3VectorsEngineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AimlOptions_S3VectorsEngineRef {
        OpensearchDomain_AimlOptions_S3VectorsEngineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AimlOptions_S3VectorsEngineRef {
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
pub struct OpensearchDomain_AimlOptions_ServerlessVectorAcceleration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl OpensearchDomain_AimlOptions_ServerlessVectorAcceleration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AimlOptions_ServerlessVectorAcceleration {
    type O = terrars::BlockAssignable<OpensearchDomain_AimlOptions_ServerlessVectorAcceleration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AimlOptions_ServerlessVectorAcceleration {}

impl BuildOpensearchDomain_AimlOptions_ServerlessVectorAcceleration {
    pub fn build(self) -> OpensearchDomain_AimlOptions_ServerlessVectorAcceleration {
        OpensearchDomain_AimlOptions_ServerlessVectorAcceleration { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AimlOptions_ServerlessVectorAccelerationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AimlOptions_ServerlessVectorAccelerationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AimlOptions_ServerlessVectorAccelerationRef {
        OpensearchDomain_AimlOptions_ServerlessVectorAccelerationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AimlOptions_ServerlessVectorAccelerationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchDomain_AimlOptionsDynamic {
    natural_language_query_generation_options: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions>>,
    s3_vectors_engine: std::option::Option<terrars::DynamicBlock<OpensearchDomain_AimlOptions_S3VectorsEngine>>,
    serverless_vector_acceleration: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_AimlOptions_ServerlessVectorAcceleration>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_AimlOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    natural_language_query_generation_options: std
    ::option
    ::Option<std::vec::Vec<OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_vectors_engine: std::option::Option<std::vec::Vec<OpensearchDomain_AimlOptions_S3VectorsEngine>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serverless_vector_acceleration: std
    ::option
    ::Option<std::vec::Vec<OpensearchDomain_AimlOptions_ServerlessVectorAcceleration>>,
    dynamic: OpensearchDomain_AimlOptionsDynamic,
}

impl OpensearchDomain_AimlOptions {
    #[doc = "Set the field `natural_language_query_generation_options`.\n"]
    pub fn set_natural_language_query_generation_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.natural_language_query_generation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.natural_language_query_generation_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_vectors_engine`.\n"]
    pub fn set_s3_vectors_engine(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_AimlOptions_S3VectorsEngine>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_vectors_engine = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_vectors_engine = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_vector_acceleration`.\n"]
    pub fn set_serverless_vector_acceleration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpensearchDomain_AimlOptions_ServerlessVectorAcceleration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serverless_vector_acceleration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serverless_vector_acceleration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AimlOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_AimlOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AimlOptions {}

impl BuildOpensearchDomain_AimlOptions {
    pub fn build(self) -> OpensearchDomain_AimlOptions {
        OpensearchDomain_AimlOptions {
            natural_language_query_generation_options: core::default::Default::default(),
            s3_vectors_engine: core::default::Default::default(),
            serverless_vector_acceleration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AimlOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AimlOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_AimlOptionsRef {
        OpensearchDomain_AimlOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AimlOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `natural_language_query_generation_options` after provisioning.\n"]
    pub fn natural_language_query_generation_options(
        &self,
    ) -> terrars::ListRef<OpensearchDomain_AimlOptions_NaturalLanguageQueryGenerationOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.natural_language_query_generation_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_vectors_engine` after provisioning.\n"]
    pub fn s3_vectors_engine(&self) -> terrars::ListRef<OpensearchDomain_AimlOptions_S3VectorsEngineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_vectors_engine", self.base))
    }

    #[doc = "Get a reference to the value of field `serverless_vector_acceleration` after provisioning.\n"]
    pub fn serverless_vector_acceleration(
        &self,
    ) -> terrars::ListRef<OpensearchDomain_AimlOptions_ServerlessVectorAccelerationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serverless_vector_acceleration", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration { }

impl terrars::ToListMappable for OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    type O = terrars::BlockAssignable<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildOpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    pub fn build(self) -> OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
        OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
        OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
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

#[derive(Serialize, Default)]
struct OpensearchDomain_AutoTuneOptions_MaintenanceScheduleDynamic {
    duration: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    cron_expression_for_recurrence: terrars::PrimField<std::string::String>,
    start_at: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<std::vec::Vec<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
    dynamic: OpensearchDomain_AutoTuneOptions_MaintenanceScheduleDynamic,
}

impl OpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.duration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    type O = terrars::BlockAssignable<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    #[doc = ""]
    pub cron_expression_for_recurrence: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_at: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
    pub fn build(self) -> OpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
        OpensearchDomain_AutoTuneOptions_MaintenanceSchedule {
            cron_expression_for_recurrence: self.cron_expression_for_recurrence,
            start_at: self.start_at,
            duration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
        OpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_expression_for_recurrence` after provisioning.\n"]
    pub fn cron_expression_for_recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_expression_for_recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `start_at` after provisioning.\n"]
    pub fn start_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_at", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::ListRef<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchDomain_AutoTuneOptionsDynamic {
    maintenance_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_AutoTuneOptions {
    desired_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_on_disable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_off_peak_window: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule: std::option::Option<std::vec::Vec<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
    dynamic: OpensearchDomain_AutoTuneOptionsDynamic,
}

impl OpensearchDomain_AutoTuneOptions {
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

    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_AutoTuneOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_AutoTuneOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_AutoTuneOptions {
    #[doc = ""]
    pub desired_state: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchDomain_AutoTuneOptions {
    pub fn build(self) -> OpensearchDomain_AutoTuneOptions {
        OpensearchDomain_AutoTuneOptions {
            desired_state: self.desired_state,
            rollback_on_disable: core::default::Default::default(),
            use_off_peak_window: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_AutoTuneOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_AutoTuneOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_AutoTuneOptionsRef {
        OpensearchDomain_AutoTuneOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_AutoTuneOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.base))
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
pub struct OpensearchDomain_ClusterConfig_ColdStorageOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl OpensearchDomain_ClusterConfig_ColdStorageOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_ClusterConfig_ColdStorageOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_ClusterConfig_ColdStorageOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_ClusterConfig_ColdStorageOptions {}

impl BuildOpensearchDomain_ClusterConfig_ColdStorageOptions {
    pub fn build(self) -> OpensearchDomain_ClusterConfig_ColdStorageOptions {
        OpensearchDomain_ClusterConfig_ColdStorageOptions { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
        OpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_ClusterConfig_ColdStorageOptionsRef {
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
pub struct OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
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

impl terrars::ToListMappable for OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    type O = terrars::BlockAssignable<OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {}

impl BuildOpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
    pub fn build(self) -> OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
        OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig {
            count: core::default::Default::default(),
            enabled: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
        OpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef {
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

#[derive(Serialize, Default)]
struct OpensearchDomain_ClusterConfig_NodeOptionsDynamic {
    node_config: std::option::Option<terrars::DynamicBlock<OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_ClusterConfig_NodeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>>,
    dynamic: OpensearchDomain_ClusterConfig_NodeOptionsDynamic,
}

impl OpensearchDomain_ClusterConfig_NodeOptions {
    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_ClusterConfig_NodeOptions_NodeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_ClusterConfig_NodeOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_ClusterConfig_NodeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_ClusterConfig_NodeOptions {}

impl BuildOpensearchDomain_ClusterConfig_NodeOptions {
    pub fn build(self) -> OpensearchDomain_ClusterConfig_NodeOptions {
        OpensearchDomain_ClusterConfig_NodeOptions {
            node_type: core::default::Default::default(),
            node_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_ClusterConfig_NodeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_ClusterConfig_NodeOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_ClusterConfig_NodeOptionsRef {
        OpensearchDomain_ClusterConfig_NodeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_ClusterConfig_NodeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<OpensearchDomain_ClusterConfig_NodeOptions_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_count: std::option::Option<terrars::PrimField<f64>>,
}

impl OpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_zone_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    type O = terrars::BlockAssignable<OpensearchDomain_ClusterConfig_ZoneAwarenessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {}

impl BuildOpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
    pub fn build(self) -> OpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
        OpensearchDomain_ClusterConfig_ZoneAwarenessConfig {
            availability_zone_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
        OpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_count` after provisioning.\n"]
    pub fn availability_zone_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchDomain_ClusterConfigDynamic {
    cold_storage_options: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_ClusterConfig_ColdStorageOptions>>,
    node_options: std::option::Option<terrars::DynamicBlock<OpensearchDomain_ClusterConfig_NodeOptions>>,
    zone_awareness_config: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_ClusterConfig {
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
    warm_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_options: std::option::Option<std::vec::Vec<OpensearchDomain_ClusterConfig_ColdStorageOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_options: std::option::Option<std::vec::Vec<OpensearchDomain_ClusterConfig_NodeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_config: std::option::Option<std::vec::Vec<OpensearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
    dynamic: OpensearchDomain_ClusterConfigDynamic,
}

impl OpensearchDomain_ClusterConfig {
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

    #[doc = "Set the field `zone_awareness_enabled`.\n"]
    pub fn set_zone_awareness_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.zone_awareness_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cold_storage_options`.\n"]
    pub fn set_cold_storage_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_ClusterConfig_ColdStorageOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cold_storage_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cold_storage_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_options`.\n"]
    pub fn set_node_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_ClusterConfig_NodeOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zone_awareness_config`.\n"]
    pub fn set_zone_awareness_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zone_awareness_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zone_awareness_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_ClusterConfig {
    type O = terrars::BlockAssignable<OpensearchDomain_ClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_ClusterConfig {}

impl BuildOpensearchDomain_ClusterConfig {
    pub fn build(self) -> OpensearchDomain_ClusterConfig {
        OpensearchDomain_ClusterConfig {
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            multi_az_with_standby_enabled: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
            cold_storage_options: core::default::Default::default(),
            node_options: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_ClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_ClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_ClusterConfigRef {
        OpensearchDomain_ClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_ClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `zone_awareness_enabled` after provisioning.\n"]
    pub fn zone_awareness_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_awareness_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `cold_storage_options` after provisioning.\n"]
    pub fn cold_storage_options(&self) -> terrars::ListRef<OpensearchDomain_ClusterConfig_ColdStorageOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cold_storage_options", self.base))
    }

    #[doc = "Get a reference to the value of field `node_options` after provisioning.\n"]
    pub fn node_options(&self) -> terrars::ListRef<OpensearchDomain_ClusterConfig_NodeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_options", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(&self) -> terrars::ListRef<OpensearchDomain_ClusterConfig_ZoneAwarenessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_awareness_config", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_CognitoOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    identity_pool_id: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

impl OpensearchDomain_CognitoOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_CognitoOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_CognitoOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_CognitoOptions {
    #[doc = ""]
    pub identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchDomain_CognitoOptions {
    pub fn build(self) -> OpensearchDomain_CognitoOptions {
        OpensearchDomain_CognitoOptions {
            enabled: core::default::Default::default(),
            identity_pool_id: self.identity_pool_id,
            role_arn: self.role_arn,
            user_pool_id: self.user_pool_id,
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_CognitoOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_CognitoOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_CognitoOptionsRef {
        OpensearchDomain_CognitoOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_CognitoOptionsRef {
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
pub struct OpensearchDomain_DeploymentStrategyOptions {
    deployment_strategy: terrars::PrimField<std::string::String>,
}

impl OpensearchDomain_DeploymentStrategyOptions { }

impl terrars::ToListMappable for OpensearchDomain_DeploymentStrategyOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_DeploymentStrategyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_DeploymentStrategyOptions {
    #[doc = ""]
    pub deployment_strategy: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchDomain_DeploymentStrategyOptions {
    pub fn build(self) -> OpensearchDomain_DeploymentStrategyOptions {
        OpensearchDomain_DeploymentStrategyOptions { deployment_strategy: self.deployment_strategy }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_DeploymentStrategyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_DeploymentStrategyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_DeploymentStrategyOptionsRef {
        OpensearchDomain_DeploymentStrategyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_DeploymentStrategyOptionsRef {
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
pub struct OpensearchDomain_DomainEndpointOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_endpoint_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_endpoint_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_https: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_DomainEndpointOptions {
    #[doc = "Set the field `custom_endpoint`.\n"]
    pub fn set_custom_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_endpoint_certificate_arn`.\n"]
    pub fn set_custom_endpoint_certificate_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_endpoint_certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_endpoint_enabled`.\n"]
    pub fn set_custom_endpoint_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.custom_endpoint_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_https`.\n"]
    pub fn set_enforce_https(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce_https = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_security_policy`.\n"]
    pub fn set_tls_security_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_security_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_DomainEndpointOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_DomainEndpointOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_DomainEndpointOptions {}

impl BuildOpensearchDomain_DomainEndpointOptions {
    pub fn build(self) -> OpensearchDomain_DomainEndpointOptions {
        OpensearchDomain_DomainEndpointOptions {
            custom_endpoint: core::default::Default::default(),
            custom_endpoint_certificate_arn: core::default::Default::default(),
            custom_endpoint_enabled: core::default::Default::default(),
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_DomainEndpointOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_DomainEndpointOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_DomainEndpointOptionsRef {
        OpensearchDomain_DomainEndpointOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_DomainEndpointOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_endpoint` after provisioning.\n"]
    pub fn custom_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_endpoint_certificate_arn` after provisioning.\n"]
    pub fn custom_endpoint_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_endpoint_certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_endpoint_enabled` after provisioning.\n"]
    pub fn custom_endpoint_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_endpoint_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_https` after provisioning.\n"]
    pub fn enforce_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_https", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_security_policy` after provisioning.\n"]
    pub fn tls_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_security_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_EbsOptions {
    ebs_enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_EbsOptions {
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

impl terrars::ToListMappable for OpensearchDomain_EbsOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_EbsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_EbsOptions {
    #[doc = ""]
    pub ebs_enabled: terrars::PrimField<bool>,
}

impl BuildOpensearchDomain_EbsOptions {
    pub fn build(self) -> OpensearchDomain_EbsOptions {
        OpensearchDomain_EbsOptions {
            ebs_enabled: self.ebs_enabled,
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_EbsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_EbsOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_EbsOptionsRef {
        OpensearchDomain_EbsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_EbsOptionsRef {
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
pub struct OpensearchDomain_EncryptAtRest {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_EncryptAtRest {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_EncryptAtRest {
    type O = terrars::BlockAssignable<OpensearchDomain_EncryptAtRest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_EncryptAtRest {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildOpensearchDomain_EncryptAtRest {
    pub fn build(self) -> OpensearchDomain_EncryptAtRest {
        OpensearchDomain_EncryptAtRest {
            enabled: self.enabled,
            kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_EncryptAtRestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_EncryptAtRestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_EncryptAtRestRef {
        OpensearchDomain_EncryptAtRestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_EncryptAtRestRef {
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
pub struct OpensearchDomain_IdentityCenterOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_api_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_IdentityCenterOptions {
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

impl terrars::ToListMappable for OpensearchDomain_IdentityCenterOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_IdentityCenterOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_IdentityCenterOptions {}

impl BuildOpensearchDomain_IdentityCenterOptions {
    pub fn build(self) -> OpensearchDomain_IdentityCenterOptions {
        OpensearchDomain_IdentityCenterOptions {
            enabled_api_access: core::default::Default::default(),
            identity_center_instance_arn: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            subject_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_IdentityCenterOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_IdentityCenterOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_IdentityCenterOptionsRef {
        OpensearchDomain_IdentityCenterOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_IdentityCenterOptionsRef {
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
pub struct OpensearchDomain_LogPublishingOptions {
    cloudwatch_log_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    log_type: terrars::PrimField<std::string::String>,
}

impl OpensearchDomain_LogPublishingOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_LogPublishingOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_LogPublishingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_LogPublishingOptions {
    #[doc = ""]
    pub cloudwatch_log_group_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchDomain_LogPublishingOptions {
    pub fn build(self) -> OpensearchDomain_LogPublishingOptions {
        OpensearchDomain_LogPublishingOptions {
            cloudwatch_log_group_arn: self.cloudwatch_log_group_arn,
            enabled: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_LogPublishingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_LogPublishingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_LogPublishingOptionsRef {
        OpensearchDomain_LogPublishingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_LogPublishingOptionsRef {
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
pub struct OpensearchDomain_NodeToNodeEncryption {
    enabled: terrars::PrimField<bool>,
}

impl OpensearchDomain_NodeToNodeEncryption { }

impl terrars::ToListMappable for OpensearchDomain_NodeToNodeEncryption {
    type O = terrars::BlockAssignable<OpensearchDomain_NodeToNodeEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_NodeToNodeEncryption {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildOpensearchDomain_NodeToNodeEncryption {
    pub fn build(self) -> OpensearchDomain_NodeToNodeEncryption {
        OpensearchDomain_NodeToNodeEncryption { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_NodeToNodeEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_NodeToNodeEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_NodeToNodeEncryptionRef {
        OpensearchDomain_NodeToNodeEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_NodeToNodeEncryptionRef {
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
pub struct OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
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

impl terrars::ToListMappable for OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    type O = terrars::BlockAssignable<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {}

impl BuildOpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
    pub fn build(self) -> OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
        OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
        OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef {
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

#[derive(Serialize, Default)]
struct OpensearchDomain_OffPeakWindowOptions_OffPeakWindowDynamic {
    window_start_time: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_start_time: std
    ::option
    ::Option<std::vec::Vec<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>>,
    dynamic: OpensearchDomain_OffPeakWindowOptions_OffPeakWindowDynamic,
}

impl OpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.window_start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.window_start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    type O = terrars::BlockAssignable<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {}

impl BuildOpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
    pub fn build(self) -> OpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
        OpensearchDomain_OffPeakWindowOptions_OffPeakWindow {
            window_start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
        OpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(
        &self,
    ) -> terrars::ListRef<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow_WindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window_start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchDomain_OffPeakWindowOptionsDynamic {
    off_peak_window: std::option::Option<terrars::DynamicBlock<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow>>,
}

#[derive(Serialize)]
pub struct OpensearchDomain_OffPeakWindowOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    off_peak_window: std::option::Option<std::vec::Vec<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow>>,
    dynamic: OpensearchDomain_OffPeakWindowOptionsDynamic,
}

impl OpensearchDomain_OffPeakWindowOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `off_peak_window`.\n"]
    pub fn set_off_peak_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchDomain_OffPeakWindowOptions_OffPeakWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.off_peak_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.off_peak_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_OffPeakWindowOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_OffPeakWindowOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_OffPeakWindowOptions {}

impl BuildOpensearchDomain_OffPeakWindowOptions {
    pub fn build(self) -> OpensearchDomain_OffPeakWindowOptions {
        OpensearchDomain_OffPeakWindowOptions {
            enabled: core::default::Default::default(),
            off_peak_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_OffPeakWindowOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_OffPeakWindowOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_OffPeakWindowOptionsRef {
        OpensearchDomain_OffPeakWindowOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_OffPeakWindowOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `off_peak_window` after provisioning.\n"]
    pub fn off_peak_window(&self) -> terrars::ListRef<OpensearchDomain_OffPeakWindowOptions_OffPeakWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.off_peak_window", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomain_SnapshotOptions {
    automated_snapshot_start_hour: terrars::PrimField<f64>,
}

impl OpensearchDomain_SnapshotOptions { }

impl terrars::ToListMappable for OpensearchDomain_SnapshotOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_SnapshotOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_SnapshotOptions {
    #[doc = ""]
    pub automated_snapshot_start_hour: terrars::PrimField<f64>,
}

impl BuildOpensearchDomain_SnapshotOptions {
    pub fn build(self) -> OpensearchDomain_SnapshotOptions {
        OpensearchDomain_SnapshotOptions { automated_snapshot_start_hour: self.automated_snapshot_start_hour }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_SnapshotOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_SnapshotOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_SnapshotOptionsRef {
        OpensearchDomain_SnapshotOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_SnapshotOptionsRef {
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
pub struct OpensearchDomain_SoftwareUpdateOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_software_update_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl OpensearchDomain_SoftwareUpdateOptions {
    #[doc = "Set the field `auto_software_update_enabled`.\n"]
    pub fn set_auto_software_update_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_software_update_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchDomain_SoftwareUpdateOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_SoftwareUpdateOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_SoftwareUpdateOptions {}

impl BuildOpensearchDomain_SoftwareUpdateOptions {
    pub fn build(self) -> OpensearchDomain_SoftwareUpdateOptions {
        OpensearchDomain_SoftwareUpdateOptions { auto_software_update_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_SoftwareUpdateOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_SoftwareUpdateOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_SoftwareUpdateOptionsRef {
        OpensearchDomain_SoftwareUpdateOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_SoftwareUpdateOptionsRef {
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
pub struct OpensearchDomain_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchDomain_Timeouts {
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

impl terrars::ToListMappable for OpensearchDomain_Timeouts {
    type O = terrars::BlockAssignable<OpensearchDomain_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_Timeouts {}

impl BuildOpensearchDomain_Timeouts {
    pub fn build(self) -> OpensearchDomain_Timeouts {
        OpensearchDomain_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_TimeoutsRef {
        OpensearchDomain_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_TimeoutsRef {
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
pub struct OpensearchDomain_VpcOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl OpensearchDomain_VpcOptions {
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
}

impl terrars::ToListMappable for OpensearchDomain_VpcOptions {
    type O = terrars::BlockAssignable<OpensearchDomain_VpcOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomain_VpcOptions {}

impl BuildOpensearchDomain_VpcOptions {
    pub fn build(self) -> OpensearchDomain_VpcOptions {
        OpensearchDomain_VpcOptions {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchDomain_VpcOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchDomain_VpcOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchDomain_VpcOptionsRef {
        OpensearchDomain_VpcOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomain_VpcOptionsRef {
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
