use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElasticsearchDomainData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_security_options: std::option::Option<std::vec::Vec<ElasticsearchDomainAdvancedSecurityOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_tune_options: std::option::Option<std::vec::Vec<ElasticsearchDomainAutoTuneOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_config: std::option::Option<std::vec::Vec<ElasticsearchDomainClusterConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_options: std::option::Option<std::vec::Vec<ElasticsearchDomainCognitoOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_endpoint_options: std::option::Option<std::vec::Vec<ElasticsearchDomainDomainEndpointOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_options: std::option::Option<std::vec::Vec<ElasticsearchDomainEbsOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypt_at_rest: std::option::Option<std::vec::Vec<ElasticsearchDomainEncryptAtRestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_publishing_options: std::option::Option<std::vec::Vec<ElasticsearchDomainLogPublishingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_to_node_encryption: std::option::Option<std::vec::Vec<ElasticsearchDomainNodeToNodeEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_options: std::option::Option<std::vec::Vec<ElasticsearchDomainSnapshotOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ElasticsearchDomainTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_options: std::option::Option<std::vec::Vec<ElasticsearchDomainVpcOptionsEl>>,
    dynamic: ElasticsearchDomainDynamic,
}

struct ElasticsearchDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElasticsearchDomainData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainAutoTuneOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainClusterConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainCognitoOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainDomainEndpointOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainEbsOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainEncryptAtRestEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainLogPublishingOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainNodeToNodeEncryptionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainSnapshotOptionsEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ElasticsearchDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainVpcOptionsEl>>,
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(&self) -> terrars::ListRef<ElasticsearchDomainAdvancedSecurityOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<ElasticsearchDomainAutoTuneOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<ElasticsearchDomainClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<ElasticsearchDomainCognitoOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> terrars::ListRef<ElasticsearchDomainDomainEndpointOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<ElasticsearchDomainEbsOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> terrars::ListRef<ElasticsearchDomainEncryptAtRestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encrypt_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<ElasticsearchDomainNodeToNodeEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<ElasticsearchDomainSnapshotOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainTimeoutsElRef {
        ElasticsearchDomainTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<ElasticsearchDomainVpcOptionsElRef> {
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
            data: std::cell::RefCell::new(ElasticsearchDomainData {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(&self) -> terrars::ListRef<ElasticsearchDomainAdvancedSecurityOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<ElasticsearchDomainAutoTuneOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<ElasticsearchDomainClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<ElasticsearchDomainCognitoOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> terrars::ListRef<ElasticsearchDomainDomainEndpointOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> terrars::ListRef<ElasticsearchDomainEbsOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> terrars::ListRef<ElasticsearchDomainEncryptAtRestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encrypt_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<ElasticsearchDomainNodeToNodeEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<ElasticsearchDomainSnapshotOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainTimeoutsElRef {
        ElasticsearchDomainTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<ElasticsearchDomainVpcOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_password: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
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

impl terrars::ToListMappable for ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {}

impl BuildElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    pub fn build(self) -> ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
        ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
            master_user_arn: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            master_user_password: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
        ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
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
struct ElasticsearchDomainAdvancedSecurityOptionsElDynamic {
    master_user_options: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomainAdvancedSecurityOptionsEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_user_database_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_options: std
    ::option
    ::Option<std::vec::Vec<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
    dynamic: ElasticsearchDomainAdvancedSecurityOptionsElDynamic,
}

impl ElasticsearchDomainAdvancedSecurityOptionsEl {
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
                    ::Into<terrars::BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
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

impl terrars::ToListMappable for ElasticsearchDomainAdvancedSecurityOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainAdvancedSecurityOptionsEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomainAdvancedSecurityOptionsEl {
    pub fn build(self) -> ElasticsearchDomainAdvancedSecurityOptionsEl {
        ElasticsearchDomainAdvancedSecurityOptionsEl {
            enabled: self.enabled,
            internal_user_database_enabled: core::default::Default::default(),
            master_user_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainAdvancedSecurityOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainAdvancedSecurityOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainAdvancedSecurityOptionsElRef {
        ElasticsearchDomainAdvancedSecurityOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainAdvancedSecurityOptionsElRef {
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
    ) -> terrars::ListRef<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_user_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl { }

impl terrars::ToListMappable for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    pub fn build(self) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
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
struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDynamic {
    duration: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    cron_expression_for_recurrence: terrars::PrimField<std::string::String>,
    start_at: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<std::vec::Vec<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    dynamic: ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDynamic,
}

impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>,
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

impl terrars::ToListMappable for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = ""]
    pub cron_expression_for_recurrence: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_at: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    pub fn build(self) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
            cron_expression_for_recurrence: self.cron_expression_for_recurrence,
            start_at: self.start_at,
            duration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
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
    pub fn duration(&self) -> terrars::ListRef<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElasticsearchDomainAutoTuneOptionsElDynamic {
    maintenance_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomainAutoTuneOptionsEl {
    desired_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_on_disable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule: std::option::Option<std::vec::Vec<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    dynamic: ElasticsearchDomainAutoTuneOptionsElDynamic,
}

impl ElasticsearchDomainAutoTuneOptionsEl {
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
                    ::Into<terrars::BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
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

impl terrars::ToListMappable for ElasticsearchDomainAutoTuneOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainAutoTuneOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainAutoTuneOptionsEl {
    #[doc = ""]
    pub desired_state: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomainAutoTuneOptionsEl {
    pub fn build(self) -> ElasticsearchDomainAutoTuneOptionsEl {
        ElasticsearchDomainAutoTuneOptionsEl {
            desired_state: self.desired_state,
            rollback_on_disable: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainAutoTuneOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainAutoTuneOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainAutoTuneOptionsElRef {
        ElasticsearchDomainAutoTuneOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainAutoTuneOptionsElRef {
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
pub struct ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainClusterConfigElColdStorageOptionsEl {}

impl BuildElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    pub fn build(self) -> ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
        ElasticsearchDomainClusterConfigElColdStorageOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
        ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_zone_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {}

impl BuildElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    pub fn build(self) -> ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
        ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
            availability_zone_count: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
        ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_count` after provisioning.\n"]
    pub fn availability_zone_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElasticsearchDomainClusterConfigElDynamic {
    cold_storage_options: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
    zone_awareness_config: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomainClusterConfigEl {
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
    cold_storage_options: std::option::Option<std::vec::Vec<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_config: std::option::Option<std::vec::Vec<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    dynamic: ElasticsearchDomainClusterConfigElDynamic,
}

impl ElasticsearchDomainClusterConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
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

impl terrars::ToListMappable for ElasticsearchDomainClusterConfigEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainClusterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainClusterConfigEl {}

impl BuildElasticsearchDomainClusterConfigEl {
    pub fn build(self) -> ElasticsearchDomainClusterConfigEl {
        ElasticsearchDomainClusterConfigEl {
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

pub struct ElasticsearchDomainClusterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainClusterConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainClusterConfigElRef {
        ElasticsearchDomainClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainClusterConfigElRef {
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
    pub fn cold_storage_options(&self) -> terrars::ListRef<ElasticsearchDomainClusterConfigElColdStorageOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cold_storage_options", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(&self) -> terrars::ListRef<ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_awareness_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainCognitoOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    identity_pool_id: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

impl ElasticsearchDomainCognitoOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomainCognitoOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainCognitoOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainCognitoOptionsEl {
    #[doc = ""]
    pub identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomainCognitoOptionsEl {
    pub fn build(self) -> ElasticsearchDomainCognitoOptionsEl {
        ElasticsearchDomainCognitoOptionsEl {
            enabled: core::default::Default::default(),
            identity_pool_id: self.identity_pool_id,
            role_arn: self.role_arn,
            user_pool_id: self.user_pool_id,
        }
    }
}

pub struct ElasticsearchDomainCognitoOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainCognitoOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainCognitoOptionsElRef {
        ElasticsearchDomainCognitoOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainCognitoOptionsElRef {
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
pub struct ElasticsearchDomainDomainEndpointOptionsEl {
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

impl ElasticsearchDomainDomainEndpointOptionsEl {
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

impl terrars::ToListMappable for ElasticsearchDomainDomainEndpointOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainDomainEndpointOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainDomainEndpointOptionsEl {}

impl BuildElasticsearchDomainDomainEndpointOptionsEl {
    pub fn build(self) -> ElasticsearchDomainDomainEndpointOptionsEl {
        ElasticsearchDomainDomainEndpointOptionsEl {
            custom_endpoint: core::default::Default::default(),
            custom_endpoint_certificate_arn: core::default::Default::default(),
            custom_endpoint_enabled: core::default::Default::default(),
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainDomainEndpointOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainDomainEndpointOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainDomainEndpointOptionsElRef {
        ElasticsearchDomainDomainEndpointOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainDomainEndpointOptionsElRef {
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
pub struct ElasticsearchDomainEbsOptionsEl {
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

impl ElasticsearchDomainEbsOptionsEl {
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

impl terrars::ToListMappable for ElasticsearchDomainEbsOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainEbsOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainEbsOptionsEl {
    #[doc = ""]
    pub ebs_enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomainEbsOptionsEl {
    pub fn build(self) -> ElasticsearchDomainEbsOptionsEl {
        ElasticsearchDomainEbsOptionsEl {
            ebs_enabled: self.ebs_enabled,
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainEbsOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainEbsOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainEbsOptionsElRef {
        ElasticsearchDomainEbsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainEbsOptionsElRef {
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
pub struct ElasticsearchDomainEncryptAtRestEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomainEncryptAtRestEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomainEncryptAtRestEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainEncryptAtRestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainEncryptAtRestEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomainEncryptAtRestEl {
    pub fn build(self) -> ElasticsearchDomainEncryptAtRestEl {
        ElasticsearchDomainEncryptAtRestEl {
            enabled: self.enabled,
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainEncryptAtRestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainEncryptAtRestElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainEncryptAtRestElRef {
        ElasticsearchDomainEncryptAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainEncryptAtRestElRef {
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
pub struct ElasticsearchDomainLogPublishingOptionsEl {
    cloudwatch_log_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    log_type: terrars::PrimField<std::string::String>,
}

impl ElasticsearchDomainLogPublishingOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomainLogPublishingOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainLogPublishingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainLogPublishingOptionsEl {
    #[doc = ""]
    pub cloudwatch_log_group_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomainLogPublishingOptionsEl {
    pub fn build(self) -> ElasticsearchDomainLogPublishingOptionsEl {
        ElasticsearchDomainLogPublishingOptionsEl {
            cloudwatch_log_group_arn: self.cloudwatch_log_group_arn,
            enabled: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}

pub struct ElasticsearchDomainLogPublishingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainLogPublishingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainLogPublishingOptionsElRef {
        ElasticsearchDomainLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainLogPublishingOptionsElRef {
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
pub struct ElasticsearchDomainNodeToNodeEncryptionEl {
    enabled: terrars::PrimField<bool>,
}

impl ElasticsearchDomainNodeToNodeEncryptionEl { }

impl terrars::ToListMappable for ElasticsearchDomainNodeToNodeEncryptionEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainNodeToNodeEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainNodeToNodeEncryptionEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildElasticsearchDomainNodeToNodeEncryptionEl {
    pub fn build(self) -> ElasticsearchDomainNodeToNodeEncryptionEl {
        ElasticsearchDomainNodeToNodeEncryptionEl { enabled: self.enabled }
    }
}

pub struct ElasticsearchDomainNodeToNodeEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainNodeToNodeEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainNodeToNodeEncryptionElRef {
        ElasticsearchDomainNodeToNodeEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainNodeToNodeEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainSnapshotOptionsEl {
    automated_snapshot_start_hour: terrars::PrimField<f64>,
}

impl ElasticsearchDomainSnapshotOptionsEl { }

impl terrars::ToListMappable for ElasticsearchDomainSnapshotOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainSnapshotOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainSnapshotOptionsEl {
    #[doc = ""]
    pub automated_snapshot_start_hour: terrars::PrimField<f64>,
}

impl BuildElasticsearchDomainSnapshotOptionsEl {
    pub fn build(self) -> ElasticsearchDomainSnapshotOptionsEl {
        ElasticsearchDomainSnapshotOptionsEl { automated_snapshot_start_hour: self.automated_snapshot_start_hour }
    }
}

pub struct ElasticsearchDomainSnapshotOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainSnapshotOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainSnapshotOptionsElRef {
        ElasticsearchDomainSnapshotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainSnapshotOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_start_hour` after provisioning.\n"]
    pub fn automated_snapshot_start_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automated_snapshot_start_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomainTimeoutsEl {
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

impl terrars::ToListMappable for ElasticsearchDomainTimeoutsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainTimeoutsEl {}

impl BuildElasticsearchDomainTimeoutsEl {
    pub fn build(self) -> ElasticsearchDomainTimeoutsEl {
        ElasticsearchDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainTimeoutsElRef {
        ElasticsearchDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainTimeoutsElRef {
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
pub struct ElasticsearchDomainVpcOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ElasticsearchDomainVpcOptionsEl {
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

impl terrars::ToListMappable for ElasticsearchDomainVpcOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainVpcOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainVpcOptionsEl {}

impl BuildElasticsearchDomainVpcOptionsEl {
    pub fn build(self) -> ElasticsearchDomainVpcOptionsEl {
        ElasticsearchDomainVpcOptionsEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainVpcOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainVpcOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainVpcOptionsElRef {
        ElasticsearchDomainVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainVpcOptionsElRef {
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

#[derive(Serialize, Default)]
struct ElasticsearchDomainDynamic {
    advanced_security_options: std
    ::option
    ::Option<terrars::DynamicBlock<ElasticsearchDomainAdvancedSecurityOptionsEl>>,
    auto_tune_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainAutoTuneOptionsEl>>,
    cluster_config: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainClusterConfigEl>>,
    cognito_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainCognitoOptionsEl>>,
    domain_endpoint_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainDomainEndpointOptionsEl>>,
    ebs_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainEbsOptionsEl>>,
    encrypt_at_rest: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainEncryptAtRestEl>>,
    log_publishing_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainLogPublishingOptionsEl>>,
    node_to_node_encryption: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainNodeToNodeEncryptionEl>>,
    snapshot_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainSnapshotOptionsEl>>,
    vpc_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainVpcOptionsEl>>,
}
