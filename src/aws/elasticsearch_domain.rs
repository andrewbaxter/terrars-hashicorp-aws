#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElasticsearchDomainData_ {
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
    elasticsearch_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_security_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_AdvancedSecurityOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_tune_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_AutoTuneOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_config: std::option::Option<std::vec::Vec<ElasticsearchDomain_ClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_CognitoOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_endpoint_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_DomainEndpointOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_EbsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypt_at_rest: std::option::Option<std::vec::Vec<ElasticsearchDomain_EncryptAtRest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_publishing_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_LogPublishingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_to_node_encryption: std::option::Option<std::vec::Vec<ElasticsearchDomain_NodeToNodeEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_SnapshotOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ElasticsearchDomain_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_VpcOptions>>,
    dynamic: ElasticsearchDomainDynamic,
}

struct ElasticsearchDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElasticsearchDomainData_>,
}

#[derive(Clone)]
pub struct ElasticsearchDomain(std::rc::Rc<ElasticsearchDomain_>);

impl ElasticsearchDomain {
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

    #[doc = "Set the field `elasticsearch_version`.\n"]
    pub fn set_elasticsearch_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_version = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_AdvancedSecurityOptions>>,
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

    #[doc = "Set the field `auto_tune_options`.\n"]
    pub fn set_auto_tune_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_AutoTuneOptions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_ClusterConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_CognitoOptions>>,
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

    #[doc = "Set the field `domain_endpoint_options`.\n"]
    pub fn set_domain_endpoint_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_DomainEndpointOptions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_EbsOptions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_EncryptAtRest>>,
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

    #[doc = "Set the field `log_publishing_options`.\n"]
    pub fn set_log_publishing_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_LogPublishingOptions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_NodeToNodeEncryption>>,
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

    #[doc = "Set the field `snapshot_options`.\n"]
    pub fn set_snapshot_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_SnapshotOptions>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ElasticsearchDomain_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_VpcOptions>>,
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

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_version` after provisioning.\n"]
    pub fn elasticsearch_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kibana_endpoint` after provisioning.\n"]
    pub fn kibana_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kibana_endpoint", self.extract_ref()))
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
    pub fn advanced_security_options(&self) -> terrars::ListRef<ElasticsearchDomain_AdvancedSecurityOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<ElasticsearchDomain_AutoTuneOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<ElasticsearchDomain_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<ElasticsearchDomain_CognitoOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> terrars::ListRef<ElasticsearchDomain_DomainEndpointOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<ElasticsearchDomain_EbsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> terrars::ListRef<ElasticsearchDomain_EncryptAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encrypt_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<ElasticsearchDomain_NodeToNodeEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<ElasticsearchDomain_SnapshotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomain_TimeoutsRef {
        ElasticsearchDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<ElasticsearchDomain_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

impl terrars::Referable for ElasticsearchDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ElasticsearchDomain { }

impl terrars::ToListMappable for ElasticsearchDomain {
    type O = terrars::ListRef<ElasticsearchDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ElasticsearchDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_elasticsearch_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticsearchDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> ElasticsearchDomain {
        let out = ElasticsearchDomain(std::rc::Rc::new(ElasticsearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ElasticsearchDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_policies: core::default::Default::default(),
                advanced_options: core::default::Default::default(),
                domain_name: self.domain_name,
                elasticsearch_version: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_security_options: core::default::Default::default(),
                auto_tune_options: core::default::Default::default(),
                cluster_config: core::default::Default::default(),
                cognito_options: core::default::Default::default(),
                domain_endpoint_options: core::default::Default::default(),
                ebs_options: core::default::Default::default(),
                encrypt_at_rest: core::default::Default::default(),
                log_publishing_options: core::default::Default::default(),
                node_to_node_encryption: core::default::Default::default(),
                snapshot_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticsearchDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ElasticsearchDomainRef {
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

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_version` after provisioning.\n"]
    pub fn elasticsearch_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kibana_endpoint` after provisioning.\n"]
    pub fn kibana_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kibana_endpoint", self.extract_ref()))
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
    pub fn advanced_security_options(&self) -> terrars::ListRef<ElasticsearchDomain_AdvancedSecurityOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<ElasticsearchDomain_AutoTuneOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<ElasticsearchDomain_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<ElasticsearchDomain_CognitoOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> terrars::ListRef<ElasticsearchDomain_DomainEndpointOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<ElasticsearchDomain_EbsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> terrars::ListRef<ElasticsearchDomain_EncryptAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encrypt_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<ElasticsearchDomain_NodeToNodeEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<ElasticsearchDomain_SnapshotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomain_TimeoutsRef {
        ElasticsearchDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<ElasticsearchDomain_VpcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ElasticsearchDomainDynamic {
    advanced_security_options: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomain_AdvancedSecurityOptions>>,
    auto_tune_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_AutoTuneOptions>>,
    cluster_config: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_ClusterConfig>>,
    cognito_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_CognitoOptions>>,
    domain_endpoint_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_DomainEndpointOptions>>,
    ebs_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_EbsOptions>>,
    encrypt_at_rest: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_EncryptAtRest>>,
    log_publishing_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_LogPublishingOptions>>,
    node_to_node_encryption: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_NodeToNodeEncryption>>,
    snapshot_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_SnapshotOptions>>,
    vpc_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomain_VpcOptions>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_password: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {
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

impl terrars::ToListMappable for ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {}

impl BuildElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {
    pub fn build(self) -> ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {
        ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions {
            master_user_arn: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            master_user_password: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
        ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef {
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
struct ElasticsearchDomain_AdvancedSecurityOptionsDynamic {
    master_user_options: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_AdvancedSecurityOptions {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_user_database_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_options: std
    ::option
    ::Option<std::vec::Vec<ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions>>,
    dynamic: ElasticsearchDomain_AdvancedSecurityOptionsDynamic,
}

impl ElasticsearchDomain_AdvancedSecurityOptions {
    #[doc = "Set the field `internal_user_database_enabled`.\n"]
    pub fn set_internal_user_database_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_user_database_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_user_options`.\n"]
    pub fn set_master_user_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptions>>,
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

impl terrars::ToListMappable for ElasticsearchDomain_AdvancedSecurityOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_AdvancedSecurityOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_AdvancedSecurityOptions {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomain_AdvancedSecurityOptions {
    pub fn build(self) -> ElasticsearchDomain_AdvancedSecurityOptions {
        ElasticsearchDomain_AdvancedSecurityOptions {
            enabled: self.enabled,
            internal_user_database_enabled: core::default::Default::default(),
            master_user_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_AdvancedSecurityOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_AdvancedSecurityOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomain_AdvancedSecurityOptionsRef {
        ElasticsearchDomain_AdvancedSecurityOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_AdvancedSecurityOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_user_database_enabled` after provisioning.\n"]
    pub fn internal_user_database_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_user_database_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `master_user_options` after provisioning.\n"]
    pub fn master_user_options(
        &self,
    ) -> terrars::ListRef<ElasticsearchDomain_AdvancedSecurityOptions_MasterUserOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_user_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration { }

impl terrars::ToListMappable for ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    type O = terrars::BlockAssignable<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
    pub fn build(self) -> ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
        ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
        ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef {
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
struct ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleDynamic {
    duration: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
    cron_expression_for_recurrence: terrars::PrimField<std::string::String>,
    start_at: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<std::vec::Vec<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>>,
    dynamic: ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleDynamic,
}

impl ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_Duration>,
                    >,
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

impl terrars::ToListMappable for ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
    type O = terrars::BlockAssignable<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
    #[doc = ""]
    pub cron_expression_for_recurrence: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_at: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
    pub fn build(self) -> ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
        ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule {
            cron_expression_for_recurrence: self.cron_expression_for_recurrence,
            start_at: self.start_at,
            duration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
        ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_AutoTuneOptions_MaintenanceScheduleRef {
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
    pub fn duration(&self) -> terrars::ListRef<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule_DurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElasticsearchDomain_AutoTuneOptionsDynamic {
    maintenance_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_AutoTuneOptions {
    desired_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_on_disable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule: std::option::Option<std::vec::Vec<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
    dynamic: ElasticsearchDomain_AutoTuneOptionsDynamic,
}

impl ElasticsearchDomain_AutoTuneOptions {
    #[doc = "Set the field `rollback_on_disable`.\n"]
    pub fn set_rollback_on_disable(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rollback_on_disable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ElasticsearchDomain_AutoTuneOptions_MaintenanceSchedule>>,
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

impl terrars::ToListMappable for ElasticsearchDomain_AutoTuneOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_AutoTuneOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_AutoTuneOptions {
    #[doc = ""]
    pub desired_state: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomain_AutoTuneOptions {
    pub fn build(self) -> ElasticsearchDomain_AutoTuneOptions {
        ElasticsearchDomain_AutoTuneOptions {
            desired_state: self.desired_state,
            rollback_on_disable: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_AutoTuneOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_AutoTuneOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_AutoTuneOptionsRef {
        ElasticsearchDomain_AutoTuneOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_AutoTuneOptionsRef {
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
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_ClusterConfig_ColdStorageOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ElasticsearchDomain_ClusterConfig_ColdStorageOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomain_ClusterConfig_ColdStorageOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_ClusterConfig_ColdStorageOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_ClusterConfig_ColdStorageOptions {}

impl BuildElasticsearchDomain_ClusterConfig_ColdStorageOptions {
    pub fn build(self) -> ElasticsearchDomain_ClusterConfig_ColdStorageOptions {
        ElasticsearchDomain_ClusterConfig_ColdStorageOptions { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_ClusterConfig_ColdStorageOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_ClusterConfig_ColdStorageOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomain_ClusterConfig_ColdStorageOptionsRef {
        ElasticsearchDomain_ClusterConfig_ColdStorageOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_ClusterConfig_ColdStorageOptionsRef {
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
pub struct ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_zone_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {
    type O = terrars::BlockAssignable<ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {}

impl BuildElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {
    pub fn build(self) -> ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {
        ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig {
            availability_zone_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
        ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfigRef {
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
struct ElasticsearchDomain_ClusterConfigDynamic {
    cold_storage_options: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomain_ClusterConfig_ColdStorageOptions>>,
    zone_awareness_config: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_ClusterConfig {
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
    warm_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_options: std::option::Option<std::vec::Vec<ElasticsearchDomain_ClusterConfig_ColdStorageOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_config: std::option::Option<std::vec::Vec<ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
    dynamic: ElasticsearchDomain_ClusterConfigDynamic,
}

impl ElasticsearchDomain_ClusterConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_ClusterConfig_ColdStorageOptions>>,
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

    #[doc = "Set the field `zone_awareness_config`.\n"]
    pub fn set_zone_awareness_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfig>>,
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

impl terrars::ToListMappable for ElasticsearchDomain_ClusterConfig {
    type O = terrars::BlockAssignable<ElasticsearchDomain_ClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_ClusterConfig {}

impl BuildElasticsearchDomain_ClusterConfig {
    pub fn build(self) -> ElasticsearchDomain_ClusterConfig {
        ElasticsearchDomain_ClusterConfig {
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
            cold_storage_options: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_ClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_ClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_ClusterConfigRef {
        ElasticsearchDomain_ClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_ClusterConfigRef {
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
    pub fn cold_storage_options(&self) -> terrars::ListRef<ElasticsearchDomain_ClusterConfig_ColdStorageOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cold_storage_options", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(&self) -> terrars::ListRef<ElasticsearchDomain_ClusterConfig_ZoneAwarenessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_awareness_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomain_CognitoOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    identity_pool_id: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

impl ElasticsearchDomain_CognitoOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomain_CognitoOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_CognitoOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_CognitoOptions {
    #[doc = ""]
    pub identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomain_CognitoOptions {
    pub fn build(self) -> ElasticsearchDomain_CognitoOptions {
        ElasticsearchDomain_CognitoOptions {
            enabled: core::default::Default::default(),
            identity_pool_id: self.identity_pool_id,
            role_arn: self.role_arn,
            user_pool_id: self.user_pool_id,
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_CognitoOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_CognitoOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_CognitoOptionsRef {
        ElasticsearchDomain_CognitoOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_CognitoOptionsRef {
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
pub struct ElasticsearchDomain_DomainEndpointOptions {
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

impl ElasticsearchDomain_DomainEndpointOptions {
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

impl terrars::ToListMappable for ElasticsearchDomain_DomainEndpointOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_DomainEndpointOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_DomainEndpointOptions {}

impl BuildElasticsearchDomain_DomainEndpointOptions {
    pub fn build(self) -> ElasticsearchDomain_DomainEndpointOptions {
        ElasticsearchDomain_DomainEndpointOptions {
            custom_endpoint: core::default::Default::default(),
            custom_endpoint_certificate_arn: core::default::Default::default(),
            custom_endpoint_enabled: core::default::Default::default(),
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_DomainEndpointOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_DomainEndpointOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_DomainEndpointOptionsRef {
        ElasticsearchDomain_DomainEndpointOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_DomainEndpointOptionsRef {
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
pub struct ElasticsearchDomain_EbsOptions {
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

impl ElasticsearchDomain_EbsOptions {
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

impl terrars::ToListMappable for ElasticsearchDomain_EbsOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_EbsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_EbsOptions {
    #[doc = ""]
    pub ebs_enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomain_EbsOptions {
    pub fn build(self) -> ElasticsearchDomain_EbsOptions {
        ElasticsearchDomain_EbsOptions {
            ebs_enabled: self.ebs_enabled,
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_EbsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_EbsOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_EbsOptionsRef {
        ElasticsearchDomain_EbsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_EbsOptionsRef {
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
pub struct ElasticsearchDomain_EncryptAtRest {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomain_EncryptAtRest {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomain_EncryptAtRest {
    type O = terrars::BlockAssignable<ElasticsearchDomain_EncryptAtRest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_EncryptAtRest {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomain_EncryptAtRest {
    pub fn build(self) -> ElasticsearchDomain_EncryptAtRest {
        ElasticsearchDomain_EncryptAtRest {
            enabled: self.enabled,
            kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_EncryptAtRestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_EncryptAtRestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_EncryptAtRestRef {
        ElasticsearchDomain_EncryptAtRestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_EncryptAtRestRef {
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
pub struct ElasticsearchDomain_LogPublishingOptions {
    cloudwatch_log_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    log_type: terrars::PrimField<std::string::String>,
}

impl ElasticsearchDomain_LogPublishingOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomain_LogPublishingOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_LogPublishingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_LogPublishingOptions {
    #[doc = ""]
    pub cloudwatch_log_group_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomain_LogPublishingOptions {
    pub fn build(self) -> ElasticsearchDomain_LogPublishingOptions {
        ElasticsearchDomain_LogPublishingOptions {
            cloudwatch_log_group_arn: self.cloudwatch_log_group_arn,
            enabled: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_LogPublishingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_LogPublishingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_LogPublishingOptionsRef {
        ElasticsearchDomain_LogPublishingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_LogPublishingOptionsRef {
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
pub struct ElasticsearchDomain_NodeToNodeEncryption {
    enabled: terrars::PrimField<bool>,
}

impl ElasticsearchDomain_NodeToNodeEncryption { }

impl terrars::ToListMappable for ElasticsearchDomain_NodeToNodeEncryption {
    type O = terrars::BlockAssignable<ElasticsearchDomain_NodeToNodeEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_NodeToNodeEncryption {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomain_NodeToNodeEncryption {
    pub fn build(self) -> ElasticsearchDomain_NodeToNodeEncryption {
        ElasticsearchDomain_NodeToNodeEncryption { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_NodeToNodeEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_NodeToNodeEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_NodeToNodeEncryptionRef {
        ElasticsearchDomain_NodeToNodeEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_NodeToNodeEncryptionRef {
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
pub struct ElasticsearchDomain_SnapshotOptions {
    automated_snapshot_start_hour: terrars::PrimField<f64>,
}

impl ElasticsearchDomain_SnapshotOptions { }

impl terrars::ToListMappable for ElasticsearchDomain_SnapshotOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_SnapshotOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_SnapshotOptions {
    #[doc = ""]
    pub automated_snapshot_start_hour: terrars::PrimField<f64>,
}

impl BuildElasticsearchDomain_SnapshotOptions {
    pub fn build(self) -> ElasticsearchDomain_SnapshotOptions {
        ElasticsearchDomain_SnapshotOptions { automated_snapshot_start_hour: self.automated_snapshot_start_hour }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_SnapshotOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_SnapshotOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_SnapshotOptionsRef {
        ElasticsearchDomain_SnapshotOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_SnapshotOptionsRef {
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
pub struct ElasticsearchDomain_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomain_Timeouts {
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

impl terrars::ToListMappable for ElasticsearchDomain_Timeouts {
    type O = terrars::BlockAssignable<ElasticsearchDomain_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_Timeouts {}

impl BuildElasticsearchDomain_Timeouts {
    pub fn build(self) -> ElasticsearchDomain_Timeouts {
        ElasticsearchDomain_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_TimeoutsRef {
        ElasticsearchDomain_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_TimeoutsRef {
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
pub struct ElasticsearchDomain_VpcOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ElasticsearchDomain_VpcOptions {
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

impl terrars::ToListMappable for ElasticsearchDomain_VpcOptions {
    type O = terrars::BlockAssignable<ElasticsearchDomain_VpcOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomain_VpcOptions {}

impl BuildElasticsearchDomain_VpcOptions {
    pub fn build(self) -> ElasticsearchDomain_VpcOptions {
        ElasticsearchDomain_VpcOptions {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ElasticsearchDomain_VpcOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomain_VpcOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomain_VpcOptionsRef {
        ElasticsearchDomain_VpcOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomain_VpcOptionsRef {
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
