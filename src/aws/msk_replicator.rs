#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskReplicatorData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    replicator_name: terrars::PrimField<std::string::String>,
    service_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_cluster: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery: std::option::Option<std::vec::Vec<MskReplicator_LogDelivery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_info_list: std::option::Option<std::vec::Vec<MskReplicator_ReplicationInfoList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskReplicator_Timeouts>,
    dynamic: MskReplicatorDynamic,
}

struct MskReplicator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskReplicatorData_>,
}

#[derive(Clone)]
pub struct MskReplicator(std::rc::Rc<MskReplicator_>);

impl MskReplicator {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `kafka_cluster`.\n"]
    pub fn set_kafka_cluster(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_delivery`.\n"]
    pub fn set_log_delivery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_LogDelivery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_info_list`.\n"]
    pub fn set_replication_info_list(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_ReplicationInfoList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_info_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_info_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskReplicator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicator_name` after provisioning.\n"]
    pub fn replicator_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicator_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> terrars::ListRef<MskReplicator_KafkaClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> terrars::ListRef<MskReplicator_LogDeliveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_info_list` after provisioning.\n"]
    pub fn replication_info_list(&self) -> terrars::ListRef<MskReplicator_ReplicationInfoListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_info_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskReplicator_TimeoutsRef {
        MskReplicator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MskReplicator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MskReplicator { }

impl terrars::ToListMappable for MskReplicator {
    type O = terrars::ListRef<MskReplicatorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MskReplicator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_msk_replicator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskReplicator {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub replicator_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator {
    pub fn build(self, stack: &mut terrars::Stack) -> MskReplicator {
        let out = MskReplicator(std::rc::Rc::new(MskReplicator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MskReplicatorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replicator_name: self.replicator_name,
                service_execution_role_arn: self.service_execution_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                kafka_cluster: core::default::Default::default(),
                log_delivery: core::default::Default::default(),
                replication_info_list: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskReplicatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicatorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MskReplicatorRef {
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

    #[doc = "Get a reference to the value of field `current_version` after provisioning.\n"]
    pub fn current_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicator_name` after provisioning.\n"]
    pub fn replicator_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicator_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> terrars::ListRef<MskReplicator_KafkaClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> terrars::ListRef<MskReplicator_LogDeliveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_info_list` after provisioning.\n"]
    pub fn replication_info_list(&self) -> terrars::ListRef<MskReplicator_ReplicationInfoListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_info_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskReplicator_TimeoutsRef {
        MskReplicator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MskReplicatorDynamic {
    kafka_cluster: std::option::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster>>,
    log_delivery: std::option::Option<terrars::DynamicBlock<MskReplicator_LogDelivery>>,
    replication_info_list: std::option::Option<terrars::DynamicBlock<MskReplicator_ReplicationInfoList>>,
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_AmazonMskCluster {
    msk_cluster_arn: terrars::PrimField<std::string::String>,
}

impl MskReplicator_KafkaCluster_AmazonMskCluster { }

impl terrars::ToListMappable for MskReplicator_KafkaCluster_AmazonMskCluster {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_AmazonMskCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_AmazonMskCluster {
    #[doc = ""]
    pub msk_cluster_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator_KafkaCluster_AmazonMskCluster {
    pub fn build(self) -> MskReplicator_KafkaCluster_AmazonMskCluster {
        MskReplicator_KafkaCluster_AmazonMskCluster { msk_cluster_arn: self.msk_cluster_arn }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_AmazonMskClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_AmazonMskClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_KafkaCluster_AmazonMskClusterRef {
        MskReplicator_KafkaCluster_AmazonMskClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_AmazonMskClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `msk_cluster_arn` after provisioning.\n"]
    pub fn msk_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.msk_cluster_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_ApacheKafkaCluster {
    apache_kafka_cluster_id: terrars::PrimField<std::string::String>,
    bootstrap_broker_string: terrars::PrimField<std::string::String>,
}

impl MskReplicator_KafkaCluster_ApacheKafkaCluster { }

impl terrars::ToListMappable for MskReplicator_KafkaCluster_ApacheKafkaCluster {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_ApacheKafkaCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_ApacheKafkaCluster {
    #[doc = ""]
    pub apache_kafka_cluster_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bootstrap_broker_string: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator_KafkaCluster_ApacheKafkaCluster {
    pub fn build(self) -> MskReplicator_KafkaCluster_ApacheKafkaCluster {
        MskReplicator_KafkaCluster_ApacheKafkaCluster {
            apache_kafka_cluster_id: self.apache_kafka_cluster_id,
            bootstrap_broker_string: self.bootstrap_broker_string,
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_ApacheKafkaClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_ApacheKafkaClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_KafkaCluster_ApacheKafkaClusterRef {
        MskReplicator_KafkaCluster_ApacheKafkaClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_ApacheKafkaClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apache_kafka_cluster_id` after provisioning.\n"]
    pub fn apache_kafka_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apache_kafka_cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bootstrap_broker_string` after provisioning.\n"]
    pub fn bootstrap_broker_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_broker_string", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_ClientAuthentication_Mtls {
    secret_arn: terrars::PrimField<std::string::String>,
}

impl MskReplicator_KafkaCluster_ClientAuthentication_Mtls { }

impl terrars::ToListMappable for MskReplicator_KafkaCluster_ClientAuthentication_Mtls {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_ClientAuthentication_Mtls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_ClientAuthentication_Mtls {
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator_KafkaCluster_ClientAuthentication_Mtls {
    pub fn build(self) -> MskReplicator_KafkaCluster_ClientAuthentication_Mtls {
        MskReplicator_KafkaCluster_ClientAuthentication_Mtls { secret_arn: self.secret_arn }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_ClientAuthentication_MtlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_ClientAuthentication_MtlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_KafkaCluster_ClientAuthentication_MtlsRef {
        MskReplicator_KafkaCluster_ClientAuthentication_MtlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_ClientAuthentication_MtlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_ClientAuthentication_SaslScram {
    mechanism: terrars::PrimField<std::string::String>,
    secret_arn: terrars::PrimField<std::string::String>,
}

impl MskReplicator_KafkaCluster_ClientAuthentication_SaslScram { }

impl terrars::ToListMappable for MskReplicator_KafkaCluster_ClientAuthentication_SaslScram {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_ClientAuthentication_SaslScram>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_ClientAuthentication_SaslScram {
    #[doc = ""]
    pub mechanism: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator_KafkaCluster_ClientAuthentication_SaslScram {
    pub fn build(self) -> MskReplicator_KafkaCluster_ClientAuthentication_SaslScram {
        MskReplicator_KafkaCluster_ClientAuthentication_SaslScram {
            mechanism: self.mechanism,
            secret_arn: self.secret_arn,
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_ClientAuthentication_SaslScramRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_ClientAuthentication_SaslScramRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_KafkaCluster_ClientAuthentication_SaslScramRef {
        MskReplicator_KafkaCluster_ClientAuthentication_SaslScramRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_ClientAuthentication_SaslScramRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mechanism` after provisioning.\n"]
    pub fn mechanism(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mechanism", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskReplicator_KafkaCluster_ClientAuthenticationDynamic {
    mtls: std::option::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_ClientAuthentication_Mtls>>,
    sasl_scram: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_ClientAuthentication_SaslScram>>,
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_ClientAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_ClientAuthentication_Mtls>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_scram: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_ClientAuthentication_SaslScram>>,
    dynamic: MskReplicator_KafkaCluster_ClientAuthenticationDynamic,
}

impl MskReplicator_KafkaCluster_ClientAuthentication {
    #[doc = "Set the field `mtls`.\n"]
    pub fn set_mtls(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_ClientAuthentication_Mtls>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mtls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mtls = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sasl_scram`.\n"]
    pub fn set_sasl_scram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_ClientAuthentication_SaslScram>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sasl_scram = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl_scram = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskReplicator_KafkaCluster_ClientAuthentication {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_ClientAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_ClientAuthentication {}

impl BuildMskReplicator_KafkaCluster_ClientAuthentication {
    pub fn build(self) -> MskReplicator_KafkaCluster_ClientAuthentication {
        MskReplicator_KafkaCluster_ClientAuthentication {
            mtls: core::default::Default::default(),
            sasl_scram: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_ClientAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_ClientAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_KafkaCluster_ClientAuthenticationRef {
        MskReplicator_KafkaCluster_ClientAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_ClientAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mtls` after provisioning.\n"]
    pub fn mtls(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_ClientAuthentication_MtlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mtls", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_scram` after provisioning.\n"]
    pub fn sasl_scram(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_ClientAuthentication_SaslScramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl_scram", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_EncryptionInTransit {
    root_ca_certificate: terrars::PrimField<std::string::String>,
}

impl MskReplicator_KafkaCluster_EncryptionInTransit { }

impl terrars::ToListMappable for MskReplicator_KafkaCluster_EncryptionInTransit {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_EncryptionInTransit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_EncryptionInTransit {
    #[doc = ""]
    pub root_ca_certificate: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator_KafkaCluster_EncryptionInTransit {
    pub fn build(self) -> MskReplicator_KafkaCluster_EncryptionInTransit {
        MskReplicator_KafkaCluster_EncryptionInTransit { root_ca_certificate: self.root_ca_certificate }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_EncryptionInTransitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_EncryptionInTransitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_KafkaCluster_EncryptionInTransitRef {
        MskReplicator_KafkaCluster_EncryptionInTransitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_EncryptionInTransitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `root_ca_certificate` after provisioning.\n"]
    pub fn root_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_ca_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster_VpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MskReplicator_KafkaCluster_VpcConfig {
    #[doc = "Set the field `security_groups_ids`.\n"]
    pub fn set_security_groups_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_KafkaCluster_VpcConfig {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster_VpcConfig {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskReplicator_KafkaCluster_VpcConfig {
    pub fn build(self) -> MskReplicator_KafkaCluster_VpcConfig {
        MskReplicator_KafkaCluster_VpcConfig {
            security_groups_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaCluster_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaCluster_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskReplicator_KafkaCluster_VpcConfigRef {
        MskReplicator_KafkaCluster_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaCluster_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups_ids` after provisioning.\n"]
    pub fn security_groups_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskReplicator_KafkaClusterDynamic {
    amazon_msk_cluster: std::option::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_AmazonMskCluster>>,
    apache_kafka_cluster: std::option::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_ApacheKafkaCluster>>,
    client_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_ClientAuthentication>>,
    encryption_in_transit: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_EncryptionInTransit>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<MskReplicator_KafkaCluster_VpcConfig>>,
}

#[derive(Serialize)]
pub struct MskReplicator_KafkaCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_msk_cluster: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_AmazonMskCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apache_kafka_cluster: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_ApacheKafkaCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_ClientAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_in_transit: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_EncryptionInTransit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<MskReplicator_KafkaCluster_VpcConfig>>,
    dynamic: MskReplicator_KafkaClusterDynamic,
}

impl MskReplicator_KafkaCluster {
    #[doc = "Set the field `amazon_msk_cluster`.\n"]
    pub fn set_amazon_msk_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_AmazonMskCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_msk_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_msk_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `apache_kafka_cluster`.\n"]
    pub fn set_apache_kafka_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_ApacheKafkaCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apache_kafka_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apache_kafka_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_ClientAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_in_transit`.\n"]
    pub fn set_encryption_in_transit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_EncryptionInTransit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_in_transit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_in_transit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_KafkaCluster_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskReplicator_KafkaCluster {
    type O = terrars::BlockAssignable<MskReplicator_KafkaCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_KafkaCluster {}

impl BuildMskReplicator_KafkaCluster {
    pub fn build(self) -> MskReplicator_KafkaCluster {
        MskReplicator_KafkaCluster {
            amazon_msk_cluster: core::default::Default::default(),
            apache_kafka_cluster: core::default::Default::default(),
            client_authentication: core::default::Default::default(),
            encryption_in_transit: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_KafkaClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_KafkaClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskReplicator_KafkaClusterRef {
        MskReplicator_KafkaClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_KafkaClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_msk_cluster` after provisioning.\n"]
    pub fn amazon_msk_cluster(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_AmazonMskClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_msk_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `apache_kafka_cluster` after provisioning.\n"]
    pub fn apache_kafka_cluster(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_ApacheKafkaClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apache_kafka_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_in_transit` after provisioning.\n"]
    pub fn encryption_in_transit(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_EncryptionInTransitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_in_transit", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<MskReplicator_KafkaCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
    type O = terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
    pub fn build(self) -> MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
        MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogsRef {
        MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
    type O = terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
    pub fn build(self) -> MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
        MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_LogDelivery_ReplicatorLogDelivery_FirehoseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_LogDelivery_ReplicatorLogDelivery_FirehoseRef {
        MskReplicator_LogDelivery_ReplicatorLogDelivery_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery_FirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
    type O = terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
    pub fn build(self) -> MskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
        MskReplicator_LogDelivery_ReplicatorLogDelivery_S3 {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_LogDelivery_ReplicatorLogDelivery_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_LogDelivery_ReplicatorLogDelivery_S3Ref {
        MskReplicator_LogDelivery_ReplicatorLogDelivery_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskReplicator_LogDelivery_ReplicatorLogDeliveryDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs>>,
    firehose: std::option::Option<terrars::DynamicBlock<MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose>>,
    s3: std::option::Option<terrars::DynamicBlock<MskReplicator_LogDelivery_ReplicatorLogDelivery_S3>>,
}

#[derive(Serialize)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDelivery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskReplicator_LogDelivery_ReplicatorLogDelivery_S3>>,
    dynamic: MskReplicator_LogDelivery_ReplicatorLogDeliveryDynamic,
}

impl MskReplicator_LogDelivery_ReplicatorLogDelivery {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery_Firehose>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery_S3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskReplicator_LogDelivery_ReplicatorLogDelivery {
    type O = terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_LogDelivery_ReplicatorLogDelivery {}

impl BuildMskReplicator_LogDelivery_ReplicatorLogDelivery {
    pub fn build(self) -> MskReplicator_LogDelivery_ReplicatorLogDelivery {
        MskReplicator_LogDelivery_ReplicatorLogDelivery {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_LogDelivery_ReplicatorLogDeliveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_LogDelivery_ReplicatorLogDeliveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_LogDelivery_ReplicatorLogDeliveryRef {
        MskReplicator_LogDelivery_ReplicatorLogDeliveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_LogDelivery_ReplicatorLogDeliveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<MskReplicator_LogDelivery_ReplicatorLogDelivery_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<MskReplicator_LogDelivery_ReplicatorLogDelivery_FirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskReplicator_LogDelivery_ReplicatorLogDelivery_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskReplicator_LogDeliveryDynamic {
    replicator_log_delivery: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_LogDelivery_ReplicatorLogDelivery>>,
}

#[derive(Serialize)]
pub struct MskReplicator_LogDelivery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicator_log_delivery: std::option::Option<std::vec::Vec<MskReplicator_LogDelivery_ReplicatorLogDelivery>>,
    dynamic: MskReplicator_LogDeliveryDynamic,
}

impl MskReplicator_LogDelivery {
    #[doc = "Set the field `replicator_log_delivery`.\n"]
    pub fn set_replicator_log_delivery(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_LogDelivery_ReplicatorLogDelivery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replicator_log_delivery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replicator_log_delivery = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskReplicator_LogDelivery {
    type O = terrars::BlockAssignable<MskReplicator_LogDelivery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_LogDelivery {}

impl BuildMskReplicator_LogDelivery {
    pub fn build(self) -> MskReplicator_LogDelivery {
        MskReplicator_LogDelivery {
            replicator_log_delivery: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_LogDeliveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_LogDeliveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskReplicator_LogDeliveryRef {
        MskReplicator_LogDeliveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_LogDeliveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicator_log_delivery` after provisioning.\n"]
    pub fn replicator_log_delivery(&self) -> terrars::ListRef<MskReplicator_LogDelivery_ReplicatorLogDeliveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicator_log_delivery", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_ReplicationInfoList_ConsumerGroupReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_offset_sync_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_groups_to_exclude: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    consumer_groups_to_replicate: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detect_and_copy_new_consumer_groups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synchronise_consumer_group_offsets: std::option::Option<terrars::PrimField<bool>>,
}

impl MskReplicator_ReplicationInfoList_ConsumerGroupReplication {
    #[doc = "Set the field `consumer_group_offset_sync_mode`.\n"]
    pub fn set_consumer_group_offset_sync_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_group_offset_sync_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_groups_to_exclude`.\n"]
    pub fn set_consumer_groups_to_exclude(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.consumer_groups_to_exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detect_and_copy_new_consumer_groups`.\n"]
    pub fn set_detect_and_copy_new_consumer_groups(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.detect_and_copy_new_consumer_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `synchronise_consumer_group_offsets`.\n"]
    pub fn set_synchronise_consumer_group_offsets(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.synchronise_consumer_group_offsets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_ReplicationInfoList_ConsumerGroupReplication {
    type O = terrars::BlockAssignable<MskReplicator_ReplicationInfoList_ConsumerGroupReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_ReplicationInfoList_ConsumerGroupReplication {
    #[doc = ""]
    pub consumer_groups_to_replicate: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskReplicator_ReplicationInfoList_ConsumerGroupReplication {
    pub fn build(self) -> MskReplicator_ReplicationInfoList_ConsumerGroupReplication {
        MskReplicator_ReplicationInfoList_ConsumerGroupReplication {
            consumer_group_offset_sync_mode: core::default::Default::default(),
            consumer_groups_to_exclude: core::default::Default::default(),
            consumer_groups_to_replicate: self.consumer_groups_to_replicate,
            detect_and_copy_new_consumer_groups: core::default::Default::default(),
            synchronise_consumer_group_offsets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_ReplicationInfoList_ConsumerGroupReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_ReplicationInfoList_ConsumerGroupReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_ReplicationInfoList_ConsumerGroupReplicationRef {
        MskReplicator_ReplicationInfoList_ConsumerGroupReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_ReplicationInfoList_ConsumerGroupReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_group_offset_sync_mode` after provisioning.\n"]
    pub fn consumer_group_offset_sync_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_offset_sync_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_groups_to_exclude` after provisioning.\n"]
    pub fn consumer_groups_to_exclude(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_groups_to_exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_groups_to_replicate` after provisioning.\n"]
    pub fn consumer_groups_to_replicate(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.consumer_groups_to_replicate", self.base))
    }

    #[doc = "Get a reference to the value of field `detect_and_copy_new_consumer_groups` after provisioning.\n"]
    pub fn detect_and_copy_new_consumer_groups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_and_copy_new_consumer_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `synchronise_consumer_group_offsets` after provisioning.\n"]
    pub fn synchronise_consumer_group_offsets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.synchronise_consumer_group_offsets", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {
    type O = terrars::BlockAssignable<MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {}

impl BuildMskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {
    pub fn build(self) -> MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {
        MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition {
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_ReplicationInfoList_TopicReplication_StartingPositionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_ReplicationInfoList_TopicReplication_StartingPositionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_ReplicationInfoList_TopicReplication_StartingPositionRef {
        MskReplicator_ReplicationInfoList_TopicReplication_StartingPositionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_ReplicationInfoList_TopicReplication_StartingPositionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {
    type O = terrars::BlockAssignable<MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {}

impl BuildMskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {
    pub fn build(self) -> MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {
        MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration {
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfigurationRef {
        MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskReplicator_ReplicationInfoList_TopicReplicationDynamic {
    starting_position: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition>>,
    topic_name_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration>>,
}

#[derive(Serialize)]
pub struct MskReplicator_ReplicationInfoList_TopicReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_access_control_lists_for_topics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_topic_configurations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detect_and_copy_new_topics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics_to_exclude: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    topics_to_replicate: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std
    ::option
    ::Option<std::vec::Vec<MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_name_configuration: std
    ::option
    ::Option<std::vec::Vec<MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration>>,
    dynamic: MskReplicator_ReplicationInfoList_TopicReplicationDynamic,
}

impl MskReplicator_ReplicationInfoList_TopicReplication {
    #[doc = "Set the field `copy_access_control_lists_for_topics`.\n"]
    pub fn set_copy_access_control_lists_for_topics(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.copy_access_control_lists_for_topics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_topic_configurations`.\n"]
    pub fn set_copy_topic_configurations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.copy_topic_configurations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detect_and_copy_new_topics`.\n"]
    pub fn set_detect_and_copy_new_topics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.detect_and_copy_new_topics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topics_to_exclude`.\n"]
    pub fn set_topics_to_exclude(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.topics_to_exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MskReplicator_ReplicationInfoList_TopicReplication_StartingPosition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.starting_position = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.starting_position = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `topic_name_configuration`.\n"]
    pub fn set_topic_name_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.topic_name_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.topic_name_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskReplicator_ReplicationInfoList_TopicReplication {
    type O = terrars::BlockAssignable<MskReplicator_ReplicationInfoList_TopicReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_ReplicationInfoList_TopicReplication {
    #[doc = ""]
    pub topics_to_replicate: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskReplicator_ReplicationInfoList_TopicReplication {
    pub fn build(self) -> MskReplicator_ReplicationInfoList_TopicReplication {
        MskReplicator_ReplicationInfoList_TopicReplication {
            copy_access_control_lists_for_topics: core::default::Default::default(),
            copy_topic_configurations: core::default::Default::default(),
            detect_and_copy_new_topics: core::default::Default::default(),
            topics_to_exclude: core::default::Default::default(),
            topics_to_replicate: self.topics_to_replicate,
            starting_position: core::default::Default::default(),
            topic_name_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_ReplicationInfoList_TopicReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_ReplicationInfoList_TopicReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskReplicator_ReplicationInfoList_TopicReplicationRef {
        MskReplicator_ReplicationInfoList_TopicReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_ReplicationInfoList_TopicReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_access_control_lists_for_topics` after provisioning.\n"]
    pub fn copy_access_control_lists_for_topics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_access_control_lists_for_topics", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_topic_configurations` after provisioning.\n"]
    pub fn copy_topic_configurations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_topic_configurations", self.base))
    }

    #[doc = "Get a reference to the value of field `detect_and_copy_new_topics` after provisioning.\n"]
    pub fn detect_and_copy_new_topics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_and_copy_new_topics", self.base))
    }

    #[doc = "Get a reference to the value of field `topics_to_exclude` after provisioning.\n"]
    pub fn topics_to_exclude(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.topics_to_exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `topics_to_replicate` after provisioning.\n"]
    pub fn topics_to_replicate(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.topics_to_replicate", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(
        &self,
    ) -> terrars::ListRef<MskReplicator_ReplicationInfoList_TopicReplication_StartingPositionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_name_configuration` after provisioning.\n"]
    pub fn topic_name_configuration(
        &self,
    ) -> terrars::ListRef<MskReplicator_ReplicationInfoList_TopicReplication_TopicNameConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic_name_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskReplicator_ReplicationInfoListDynamic {
    consumer_group_replication: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_ReplicationInfoList_ConsumerGroupReplication>>,
    topic_replication: std
    ::option
    ::Option<terrars::DynamicBlock<MskReplicator_ReplicationInfoList_TopicReplication>>,
}

#[derive(Serialize)]
pub struct MskReplicator_ReplicationInfoList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_kafka_cluster_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_kafka_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    target_compression_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_kafka_cluster_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_kafka_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_replication: std
    ::option
    ::Option<std::vec::Vec<MskReplicator_ReplicationInfoList_ConsumerGroupReplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_replication: std::option::Option<std::vec::Vec<MskReplicator_ReplicationInfoList_TopicReplication>>,
    dynamic: MskReplicator_ReplicationInfoListDynamic,
}

impl MskReplicator_ReplicationInfoList {
    #[doc = "Set the field `source_kafka_cluster_arn`.\n"]
    pub fn set_source_kafka_cluster_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_kafka_cluster_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_kafka_cluster_id`.\n"]
    pub fn set_source_kafka_cluster_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_kafka_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_kafka_cluster_arn`.\n"]
    pub fn set_target_kafka_cluster_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_kafka_cluster_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_kafka_cluster_id`.\n"]
    pub fn set_target_kafka_cluster_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_kafka_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_group_replication`.\n"]
    pub fn set_consumer_group_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskReplicator_ReplicationInfoList_ConsumerGroupReplication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consumer_group_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consumer_group_replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `topic_replication`.\n"]
    pub fn set_topic_replication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskReplicator_ReplicationInfoList_TopicReplication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.topic_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.topic_replication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskReplicator_ReplicationInfoList {
    type O = terrars::BlockAssignable<MskReplicator_ReplicationInfoList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_ReplicationInfoList {
    #[doc = ""]
    pub target_compression_type: terrars::PrimField<std::string::String>,
}

impl BuildMskReplicator_ReplicationInfoList {
    pub fn build(self) -> MskReplicator_ReplicationInfoList {
        MskReplicator_ReplicationInfoList {
            source_kafka_cluster_arn: core::default::Default::default(),
            source_kafka_cluster_id: core::default::Default::default(),
            target_compression_type: self.target_compression_type,
            target_kafka_cluster_arn: core::default::Default::default(),
            target_kafka_cluster_id: core::default::Default::default(),
            consumer_group_replication: core::default::Default::default(),
            topic_replication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_ReplicationInfoListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_ReplicationInfoListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskReplicator_ReplicationInfoListRef {
        MskReplicator_ReplicationInfoListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_ReplicationInfoListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_kafka_cluster_alias` after provisioning.\n"]
    pub fn source_kafka_cluster_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_kafka_cluster_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `source_kafka_cluster_arn` after provisioning.\n"]
    pub fn source_kafka_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_kafka_cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `source_kafka_cluster_id` after provisioning.\n"]
    pub fn source_kafka_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_kafka_cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `target_compression_type` after provisioning.\n"]
    pub fn target_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_kafka_cluster_alias` after provisioning.\n"]
    pub fn target_kafka_cluster_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_kafka_cluster_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `target_kafka_cluster_arn` after provisioning.\n"]
    pub fn target_kafka_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_kafka_cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_kafka_cluster_id` after provisioning.\n"]
    pub fn target_kafka_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_kafka_cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_group_replication` after provisioning.\n"]
    pub fn consumer_group_replication(
        &self,
    ) -> terrars::ListRef<MskReplicator_ReplicationInfoList_ConsumerGroupReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumer_group_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_replication` after provisioning.\n"]
    pub fn topic_replication(&self) -> terrars::ListRef<MskReplicator_ReplicationInfoList_TopicReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic_replication", self.base))
    }
}

#[derive(Serialize)]
pub struct MskReplicator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskReplicator_Timeouts {
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

impl terrars::ToListMappable for MskReplicator_Timeouts {
    type O = terrars::BlockAssignable<MskReplicator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskReplicator_Timeouts {}

impl BuildMskReplicator_Timeouts {
    pub fn build(self) -> MskReplicator_Timeouts {
        MskReplicator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskReplicator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskReplicator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskReplicator_TimeoutsRef {
        MskReplicator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskReplicator_TimeoutsRef {
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
