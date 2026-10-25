#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerFeatureGroupData_ {
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
    event_time_feature_name: terrars::PrimField<std::string::String>,
    feature_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    record_identifier_feature_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_definition: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_FeatureDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offline_store_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_OfflineStoreConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    online_store_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_OnlineStoreConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_ThroughputConfig>>,
    dynamic: SagemakerFeatureGroupDynamic,
}

struct SagemakerFeatureGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerFeatureGroupData_>,
}

#[derive(Clone)]
pub struct SagemakerFeatureGroup(std::rc::Rc<SagemakerFeatureGroup_>);

impl SagemakerFeatureGroup {
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

    #[doc = "Set the field `feature_definition`.\n"]
    pub fn set_feature_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroup_FeatureDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `offline_store_config`.\n"]
    pub fn set_offline_store_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroup_OfflineStoreConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().offline_store_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.offline_store_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `online_store_config`.\n"]
    pub fn set_online_store_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroup_OnlineStoreConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().online_store_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.online_store_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `throughput_config`.\n"]
    pub fn set_throughput_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroup_ThroughputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().throughput_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.throughput_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_time_feature_name` after provisioning.\n"]
    pub fn event_time_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_time_feature_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_group_name` after provisioning.\n"]
    pub fn feature_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_identifier_feature_name` after provisioning.\n"]
    pub fn record_identifier_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_identifier_feature_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_definition` after provisioning.\n"]
    pub fn feature_definition(&self) -> terrars::ListRef<SagemakerFeatureGroup_FeatureDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offline_store_config` after provisioning.\n"]
    pub fn offline_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OfflineStoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offline_store_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_store_config` after provisioning.\n"]
    pub fn online_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OnlineStoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.online_store_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_config` after provisioning.\n"]
    pub fn throughput_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_ThroughputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throughput_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerFeatureGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerFeatureGroup { }

impl terrars::ToListMappable for SagemakerFeatureGroup {
    type O = terrars::ListRef<SagemakerFeatureGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerFeatureGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_feature_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerFeatureGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub event_time_feature_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub feature_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_identifier_feature_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFeatureGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerFeatureGroup {
        let out = SagemakerFeatureGroup(std::rc::Rc::new(SagemakerFeatureGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerFeatureGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                event_time_feature_name: self.event_time_feature_name,
                feature_group_name: self.feature_group_name,
                id: core::default::Default::default(),
                record_identifier_feature_name: self.record_identifier_feature_name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                feature_definition: core::default::Default::default(),
                offline_store_config: core::default::Default::default(),
                online_store_config: core::default::Default::default(),
                throughput_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerFeatureGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerFeatureGroupRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_time_feature_name` after provisioning.\n"]
    pub fn event_time_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_time_feature_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_group_name` after provisioning.\n"]
    pub fn feature_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_identifier_feature_name` after provisioning.\n"]
    pub fn record_identifier_feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_identifier_feature_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_definition` after provisioning.\n"]
    pub fn feature_definition(&self) -> terrars::ListRef<SagemakerFeatureGroup_FeatureDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offline_store_config` after provisioning.\n"]
    pub fn offline_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OfflineStoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offline_store_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_store_config` after provisioning.\n"]
    pub fn online_store_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OnlineStoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.online_store_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_config` after provisioning.\n"]
    pub fn throughput_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_ThroughputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throughput_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroupDynamic {
    feature_definition: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroup_FeatureDefinition>>,
    offline_store_config: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroup_OfflineStoreConfig>>,
    online_store_config: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroup_OnlineStoreConfig>>,
    throughput_config: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroup_ThroughputConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {}

impl BuildSagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {
    pub fn build(self) -> SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {
        SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig {
            dimension: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfigRef {
        SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroup_FeatureDefinition_CollectionConfigDynamic {
    vector_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_FeatureDefinition_CollectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig>>,
    dynamic: SagemakerFeatureGroup_FeatureDefinition_CollectionConfigDynamic,
}

impl SagemakerFeatureGroup_FeatureDefinition_CollectionConfig {
    #[doc = "Set the field `vector_config`.\n"]
    pub fn set_vector_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vector_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_FeatureDefinition_CollectionConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_FeatureDefinition_CollectionConfig {}

impl BuildSagemakerFeatureGroup_FeatureDefinition_CollectionConfig {
    pub fn build(self) -> SagemakerFeatureGroup_FeatureDefinition_CollectionConfig {
        SagemakerFeatureGroup_FeatureDefinition_CollectionConfig {
            vector_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_FeatureDefinition_CollectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_FeatureDefinition_CollectionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroup_FeatureDefinition_CollectionConfigRef {
        SagemakerFeatureGroup_FeatureDefinition_CollectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_FeatureDefinition_CollectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vector_config` after provisioning.\n"]
    pub fn vector_config(
        &self,
    ) -> terrars::ListRef<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig_VectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroup_FeatureDefinitionDynamic {
    collection_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_FeatureDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig>>,
    dynamic: SagemakerFeatureGroup_FeatureDefinitionDynamic,
}

impl SagemakerFeatureGroup_FeatureDefinition {
    #[doc = "Set the field `collection_type`.\n"]
    pub fn set_collection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_name`.\n"]
    pub fn set_feature_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_type`.\n"]
    pub fn set_feature_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collection_config`.\n"]
    pub fn set_collection_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerFeatureGroup_FeatureDefinition_CollectionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collection_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_FeatureDefinition {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_FeatureDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_FeatureDefinition {}

impl BuildSagemakerFeatureGroup_FeatureDefinition {
    pub fn build(self) -> SagemakerFeatureGroup_FeatureDefinition {
        SagemakerFeatureGroup_FeatureDefinition {
            collection_type: core::default::Default::default(),
            feature_name: core::default::Default::default(),
            feature_type: core::default::Default::default(),
            collection_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_FeatureDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_FeatureDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroup_FeatureDefinitionRef {
        SagemakerFeatureGroup_FeatureDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_FeatureDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_type` after provisioning.\n"]
    pub fn collection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_name` after provisioning.\n"]
    pub fn feature_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_name", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_type` after provisioning.\n"]
    pub fn feature_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_type", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_config` after provisioning.\n"]
    pub fn collection_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_FeatureDefinition_CollectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_name`.\n"]
    pub fn set_table_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {}

impl BuildSagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {
    pub fn build(self) -> SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {
        SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig {
            catalog: core::default::Default::default(),
            database: core::default::Default::default(),
            table_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfigRef {
        SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved_output_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resolved_output_s3_uri`.\n"]
    pub fn set_resolved_output_s3_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resolved_output_s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
    pub fn build(self) -> SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
        SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig {
            kms_key_id: core::default::Default::default(),
            resolved_output_s3_uri: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfigRef {
        SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resolved_output_s3_uri` after provisioning.\n"]
    pub fn resolved_output_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved_output_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFeatureGroup_OfflineStoreConfigDynamic {
    data_catalog_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig>>,
    s3_storage_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_OfflineStoreConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_glue_table_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_catalog_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_storage_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig>>,
    dynamic: SagemakerFeatureGroup_OfflineStoreConfigDynamic,
}

impl SagemakerFeatureGroup_OfflineStoreConfig {
    #[doc = "Set the field `disable_glue_table_creation`.\n"]
    pub fn set_disable_glue_table_creation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_glue_table_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_format`.\n"]
    pub fn set_table_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_catalog_config`.\n"]
    pub fn set_data_catalog_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_catalog_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_catalog_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_storage_config`.\n"]
    pub fn set_s3_storage_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_storage_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_OfflineStoreConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_OfflineStoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_OfflineStoreConfig {}

impl BuildSagemakerFeatureGroup_OfflineStoreConfig {
    pub fn build(self) -> SagemakerFeatureGroup_OfflineStoreConfig {
        SagemakerFeatureGroup_OfflineStoreConfig {
            disable_glue_table_creation: core::default::Default::default(),
            table_format: core::default::Default::default(),
            data_catalog_config: core::default::Default::default(),
            s3_storage_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_OfflineStoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_OfflineStoreConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroup_OfflineStoreConfigRef {
        SagemakerFeatureGroup_OfflineStoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_OfflineStoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_glue_table_creation` after provisioning.\n"]
    pub fn disable_glue_table_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_glue_table_creation", self.base))
    }

    #[doc = "Get a reference to the value of field `table_format` after provisioning.\n"]
    pub fn table_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_format", self.base))
    }

    #[doc = "Get a reference to the value of field `data_catalog_config` after provisioning.\n"]
    pub fn data_catalog_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OfflineStoreConfig_DataCatalogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_catalog_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_storage_config` after provisioning.\n"]
    pub fn s3_storage_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OfflineStoreConfig_S3StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_storage_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig {}

impl BuildSagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig {
    pub fn build(self) -> SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig {
        SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig { kms_key_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfigRef {
        SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {
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

impl terrars::ToListMappable for SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {}

impl BuildSagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {
    pub fn build(self) -> SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {
        SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_OnlineStoreConfig_TtlDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_OnlineStoreConfig_TtlDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFeatureGroup_OnlineStoreConfig_TtlDurationRef {
        SagemakerFeatureGroup_OnlineStoreConfig_TtlDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_OnlineStoreConfig_TtlDurationRef {
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
struct SagemakerFeatureGroup_OnlineStoreConfigDynamic {
    security_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig>>,
    ttl_duration: std::option::Option<terrars::DynamicBlock<SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration>>,
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_OnlineStoreConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_online_store: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl_duration: std::option::Option<std::vec::Vec<SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration>>,
    dynamic: SagemakerFeatureGroup_OnlineStoreConfigDynamic,
}

impl SagemakerFeatureGroup_OnlineStoreConfig {
    #[doc = "Set the field `enable_online_store`.\n"]
    pub fn set_enable_online_store(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_online_store = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ttl_duration`.\n"]
    pub fn set_ttl_duration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFeatureGroup_OnlineStoreConfig_TtlDuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ttl_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl_duration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_OnlineStoreConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_OnlineStoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_OnlineStoreConfig {}

impl BuildSagemakerFeatureGroup_OnlineStoreConfig {
    pub fn build(self) -> SagemakerFeatureGroup_OnlineStoreConfig {
        SagemakerFeatureGroup_OnlineStoreConfig {
            enable_online_store: core::default::Default::default(),
            storage_type: core::default::Default::default(),
            security_config: core::default::Default::default(),
            ttl_duration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_OnlineStoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_OnlineStoreConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroup_OnlineStoreConfigRef {
        SagemakerFeatureGroup_OnlineStoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_OnlineStoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_online_store` after provisioning.\n"]
    pub fn enable_online_store(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_online_store", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> terrars::ListRef<SagemakerFeatureGroup_OnlineStoreConfig_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl_duration` after provisioning.\n"]
    pub fn ttl_duration(&self) -> terrars::ListRef<SagemakerFeatureGroup_OnlineStoreConfig_TtlDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFeatureGroup_ThroughputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_read_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_write_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerFeatureGroup_ThroughputConfig {
    #[doc = "Set the field `provisioned_read_capacity_units`.\n"]
    pub fn set_provisioned_read_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_read_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_write_capacity_units`.\n"]
    pub fn set_provisioned_write_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_write_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput_mode`.\n"]
    pub fn set_throughput_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.throughput_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFeatureGroup_ThroughputConfig {
    type O = terrars::BlockAssignable<SagemakerFeatureGroup_ThroughputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFeatureGroup_ThroughputConfig {}

impl BuildSagemakerFeatureGroup_ThroughputConfig {
    pub fn build(self) -> SagemakerFeatureGroup_ThroughputConfig {
        SagemakerFeatureGroup_ThroughputConfig {
            provisioned_read_capacity_units: core::default::Default::default(),
            provisioned_write_capacity_units: core::default::Default::default(),
            throughput_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFeatureGroup_ThroughputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFeatureGroup_ThroughputConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFeatureGroup_ThroughputConfigRef {
        SagemakerFeatureGroup_ThroughputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFeatureGroup_ThroughputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provisioned_read_capacity_units` after provisioning.\n"]
    pub fn provisioned_read_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_read_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_write_capacity_units` after provisioning.\n"]
    pub fn provisioned_write_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_write_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_mode", self.base))
    }
}
