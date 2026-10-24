use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOpensearchDomainData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataOpensearchDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOpensearchDomainData>,
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
    pub fn advanced_security_options(&self) -> terrars::ListRef<DataOpensearchDomainAdvancedSecurityOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<DataOpensearchDomainAutoTuneOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<DataOpensearchDomainClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<DataOpensearchDomainCognitoOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
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
    pub fn ebs_options(&self) -> terrars::ListRef<DataOpensearchDomainEbsOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> terrars::ListRef<DataOpensearchDomainEncryptionAtRestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kibana_endpoint` after provisioning.\n"]
    pub fn kibana_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kibana_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> terrars::SetRef<DataOpensearchDomainLogPublishingOptionsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<DataOpensearchDomainNodeToNodeEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processing` after provisioning.\n"]
    pub fn processing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<DataOpensearchDomainSnapshotOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<DataOpensearchDomainVpcOptionsElRef> {
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
            data: std::cell::RefCell::new(DataOpensearchDomainData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
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
    pub fn advanced_security_options(&self) -> terrars::ListRef<DataOpensearchDomainAdvancedSecurityOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_security_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> terrars::ListRef<DataOpensearchDomainAutoTuneOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_tune_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<DataOpensearchDomainClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> terrars::ListRef<DataOpensearchDomainCognitoOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
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
    pub fn ebs_options(&self) -> terrars::ListRef<DataOpensearchDomainEbsOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> terrars::ListRef<DataOpensearchDomainEncryptionAtRestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kibana_endpoint` after provisioning.\n"]
    pub fn kibana_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kibana_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> terrars::SetRef<DataOpensearchDomainLogPublishingOptionsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_publishing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> terrars::ListRef<DataOpensearchDomainNodeToNodeEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_to_node_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processing` after provisioning.\n"]
    pub fn processing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> terrars::ListRef<DataOpensearchDomainSnapshotOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> terrars::ListRef<DataOpensearchDomainVpcOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAdvancedSecurityOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_user_database_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomainAdvancedSecurityOptionsEl {
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
}

impl terrars::ToListMappable for DataOpensearchDomainAdvancedSecurityOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainAdvancedSecurityOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAdvancedSecurityOptionsEl {}

impl BuildDataOpensearchDomainAdvancedSecurityOptionsEl {
    pub fn build(self) -> DataOpensearchDomainAdvancedSecurityOptionsEl {
        DataOpensearchDomainAdvancedSecurityOptionsEl {
            enabled: core::default::Default::default(),
            internal_user_database_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAdvancedSecurityOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainAdvancedSecurityOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomainAdvancedSecurityOptionsElRef {
        DataOpensearchDomainAdvancedSecurityOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAdvancedSecurityOptionsElRef {
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
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
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

impl terrars::ToListMappable for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {}

impl BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    pub fn build(self) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
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
pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_expression_for_recurrence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std
    ::option
    ::Option<terrars::ListField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
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
                    ::Into<terrars::ListField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
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

impl terrars::ToListMappable for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {}

impl BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    pub fn build(self) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
            cron_expression_for_recurrence: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_at: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_expression_for_recurrence` after provisioning.\n"]
    pub fn cron_expression_for_recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_expression_for_recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::ListRef<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_at` after provisioning.\n"]
    pub fn start_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAutoTuneOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule: std
    ::option
    ::Option<terrars::SetField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_on_disable: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomainAutoTuneOptionsEl {
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
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
}

impl terrars::ToListMappable for DataOpensearchDomainAutoTuneOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainAutoTuneOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAutoTuneOptionsEl {}

impl BuildDataOpensearchDomainAutoTuneOptionsEl {
    pub fn build(self) -> DataOpensearchDomainAutoTuneOptionsEl {
        DataOpensearchDomainAutoTuneOptionsEl {
            desired_state: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            rollback_on_disable: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAutoTuneOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainAutoTuneOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainAutoTuneOptionsElRef {
        DataOpensearchDomainAutoTuneOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAutoTuneOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\n"]
    pub fn maintenance_schedule(&self) -> terrars::SetRef<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback_on_disable` after provisioning.\n"]
    pub fn rollback_on_disable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback_on_disable", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainClusterConfigElColdStorageOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigElColdStorageOptionsEl {}

impl BuildDataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
        DataOpensearchDomainClusterConfigElColdStorageOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
        DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_zone_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {}

impl BuildDataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
        DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
            availability_zone_count: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
        DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_count` after provisioning.\n"]
    pub fn availability_zone_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_options: std
    ::option
    ::Option<terrars::ListField<DataOpensearchDomainClusterConfigElColdStorageOptionsEl>>,
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
    zone_awareness_config: std
    ::option
    ::Option<terrars::ListField<DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_awareness_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomainClusterConfigEl {
    #[doc = "Set the field `cold_storage_options`.\n"]
    pub fn set_cold_storage_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomainClusterConfigElColdStorageOptionsEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
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

impl terrars::ToListMappable for DataOpensearchDomainClusterConfigEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainClusterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigEl {}

impl BuildDataOpensearchDomainClusterConfigEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigEl {
        DataOpensearchDomainClusterConfigEl {
            cold_storage_options: core::default::Default::default(),
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainClusterConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainClusterConfigElRef {
        DataOpensearchDomainClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cold_storage_options` after provisioning.\n"]
    pub fn cold_storage_options(&self) -> terrars::ListRef<DataOpensearchDomainClusterConfigElColdStorageOptionsElRef> {
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
    pub fn zone_awareness_config(&self) -> terrars::ListRef<DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_awareness_config", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_enabled` after provisioning.\n"]
    pub fn zone_awareness_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_awareness_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainCognitoOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomainCognitoOptionsEl {
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

impl terrars::ToListMappable for DataOpensearchDomainCognitoOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainCognitoOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainCognitoOptionsEl {}

impl BuildDataOpensearchDomainCognitoOptionsEl {
    pub fn build(self) -> DataOpensearchDomainCognitoOptionsEl {
        DataOpensearchDomainCognitoOptionsEl {
            enabled: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_pool_id: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainCognitoOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainCognitoOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainCognitoOptionsElRef {
        DataOpensearchDomainCognitoOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainCognitoOptionsElRef {
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
pub struct DataOpensearchDomainEbsOptionsEl {
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

impl DataOpensearchDomainEbsOptionsEl {
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

impl terrars::ToListMappable for DataOpensearchDomainEbsOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainEbsOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainEbsOptionsEl {}

impl BuildDataOpensearchDomainEbsOptionsEl {
    pub fn build(self) -> DataOpensearchDomainEbsOptionsEl {
        DataOpensearchDomainEbsOptionsEl {
            ebs_enabled: core::default::Default::default(),
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainEbsOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainEbsOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainEbsOptionsElRef {
        DataOpensearchDomainEbsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainEbsOptionsElRef {
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
pub struct DataOpensearchDomainEncryptionAtRestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomainEncryptionAtRestEl {
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

impl terrars::ToListMappable for DataOpensearchDomainEncryptionAtRestEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainEncryptionAtRestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainEncryptionAtRestEl {}

impl BuildDataOpensearchDomainEncryptionAtRestEl {
    pub fn build(self) -> DataOpensearchDomainEncryptionAtRestEl {
        DataOpensearchDomainEncryptionAtRestEl {
            enabled: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainEncryptionAtRestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainEncryptionAtRestElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainEncryptionAtRestElRef {
        DataOpensearchDomainEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainEncryptionAtRestElRef {
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
pub struct DataOpensearchDomainLogPublishingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomainLogPublishingOptionsEl {
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

impl terrars::ToListMappable for DataOpensearchDomainLogPublishingOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainLogPublishingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainLogPublishingOptionsEl {}

impl BuildDataOpensearchDomainLogPublishingOptionsEl {
    pub fn build(self) -> DataOpensearchDomainLogPublishingOptionsEl {
        DataOpensearchDomainLogPublishingOptionsEl {
            cloudwatch_log_group_arn: core::default::Default::default(),
            enabled: core::default::Default::default(),
            log_type: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainLogPublishingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainLogPublishingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainLogPublishingOptionsElRef {
        DataOpensearchDomainLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainLogPublishingOptionsElRef {
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
pub struct DataOpensearchDomainNodeToNodeEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOpensearchDomainNodeToNodeEncryptionEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomainNodeToNodeEncryptionEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainNodeToNodeEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainNodeToNodeEncryptionEl {}

impl BuildDataOpensearchDomainNodeToNodeEncryptionEl {
    pub fn build(self) -> DataOpensearchDomainNodeToNodeEncryptionEl {
        DataOpensearchDomainNodeToNodeEncryptionEl { enabled: core::default::Default::default() }
    }
}

pub struct DataOpensearchDomainNodeToNodeEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainNodeToNodeEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainNodeToNodeEncryptionElRef {
        DataOpensearchDomainNodeToNodeEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainNodeToNodeEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainSnapshotOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_snapshot_start_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOpensearchDomainSnapshotOptionsEl {
    #[doc = "Set the field `automated_snapshot_start_hour`.\n"]
    pub fn set_automated_snapshot_start_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.automated_snapshot_start_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOpensearchDomainSnapshotOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainSnapshotOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainSnapshotOptionsEl {}

impl BuildDataOpensearchDomainSnapshotOptionsEl {
    pub fn build(self) -> DataOpensearchDomainSnapshotOptionsEl {
        DataOpensearchDomainSnapshotOptionsEl { automated_snapshot_start_hour: core::default::Default::default() }
    }
}

pub struct DataOpensearchDomainSnapshotOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainSnapshotOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainSnapshotOptionsElRef {
        DataOpensearchDomainSnapshotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainSnapshotOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_start_hour` after provisioning.\n"]
    pub fn automated_snapshot_start_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automated_snapshot_start_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainVpcOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOpensearchDomainVpcOptionsEl {
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

impl terrars::ToListMappable for DataOpensearchDomainVpcOptionsEl {
    type O = terrars::BlockAssignable<DataOpensearchDomainVpcOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainVpcOptionsEl {}

impl BuildDataOpensearchDomainVpcOptionsEl {
    pub fn build(self) -> DataOpensearchDomainVpcOptionsEl {
        DataOpensearchDomainVpcOptionsEl {
            availability_zones: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainVpcOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchDomainVpcOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOpensearchDomainVpcOptionsElRef {
        DataOpensearchDomainVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainVpcOptionsElRef {
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
