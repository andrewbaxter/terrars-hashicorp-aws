#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticacheServiceUpdateActionsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_update_status: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

struct DataElasticacheServiceUpdateActions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataElasticacheServiceUpdateActionsData_>,
}

#[derive(Clone)]
pub struct DataElasticacheServiceUpdateActions(std::rc::Rc<DataElasticacheServiceUpdateActions_>);

impl DataElasticacheServiceUpdateActions {
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

    #[doc = "Set the field `cache_cluster_id`.\n"]
    pub fn set_cache_cluster_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cache_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_group_id`.\n"]
    pub fn set_replication_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().replication_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_update_status`.\n"]
    pub fn set_service_update_status(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().service_update_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cache_cluster_id` after provisioning.\n"]
    pub fn cache_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_update_status` after provisioning.\n"]
    pub fn service_update_status(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_update_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_actions` after provisioning.\n"]
    pub fn update_actions(&self) -> terrars::ListRef<DataElasticacheServiceUpdateActions_UpdateActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_actions", self.extract_ref()))
    }
}

impl terrars::Referable for DataElasticacheServiceUpdateActions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataElasticacheServiceUpdateActions { }

impl terrars::ToListMappable for DataElasticacheServiceUpdateActions {
    type O = terrars::ListRef<DataElasticacheServiceUpdateActionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataElasticacheServiceUpdateActions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_elasticache_service_update_actions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticacheServiceUpdateActions {
    pub tf_id: std::string::String,
}

impl BuildDataElasticacheServiceUpdateActions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataElasticacheServiceUpdateActions {
        let out = DataElasticacheServiceUpdateActions(std::rc::Rc::new(DataElasticacheServiceUpdateActions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataElasticacheServiceUpdateActionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cache_cluster_id: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_group_id: core::default::Default::default(),
                service_update_status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticacheServiceUpdateActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServiceUpdateActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataElasticacheServiceUpdateActionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cache_cluster_id` after provisioning.\n"]
    pub fn cache_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_update_status` after provisioning.\n"]
    pub fn service_update_status(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_update_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_actions` after provisioning.\n"]
    pub fn update_actions(&self) -> terrars::ListRef<DataElasticacheServiceUpdateActions_UpdateActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_actions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheServiceUpdateActions_UpdateActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recommended_apply_by_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_update_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_update_severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_update_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_update_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_action_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataElasticacheServiceUpdateActions_UpdateActions {
    #[doc = "Set the field `cache_cluster_id`.\n"]
    pub fn set_cache_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `estimated_update_time`.\n"]
    pub fn set_estimated_update_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.estimated_update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recommended_apply_by_date`.\n"]
    pub fn set_recommended_apply_by_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.recommended_apply_by_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `release_date`.\n"]
    pub fn set_release_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.release_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_group_id`.\n"]
    pub fn set_replication_group_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replication_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_update_name`.\n"]
    pub fn set_service_update_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_update_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_update_severity`.\n"]
    pub fn set_service_update_severity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_update_severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_update_status`.\n"]
    pub fn set_service_update_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_update_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_update_type`.\n"]
    pub fn set_service_update_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_update_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_action_status`.\n"]
    pub fn set_update_action_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.update_action_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheServiceUpdateActions_UpdateActions {
    type O = terrars::BlockAssignable<DataElasticacheServiceUpdateActions_UpdateActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheServiceUpdateActions_UpdateActions {}

impl BuildDataElasticacheServiceUpdateActions_UpdateActions {
    pub fn build(self) -> DataElasticacheServiceUpdateActions_UpdateActions {
        DataElasticacheServiceUpdateActions_UpdateActions {
            cache_cluster_id: core::default::Default::default(),
            engine: core::default::Default::default(),
            estimated_update_time: core::default::Default::default(),
            recommended_apply_by_date: core::default::Default::default(),
            release_date: core::default::Default::default(),
            replication_group_id: core::default::Default::default(),
            service_update_name: core::default::Default::default(),
            service_update_severity: core::default::Default::default(),
            service_update_status: core::default::Default::default(),
            service_update_type: core::default::Default::default(),
            update_action_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataElasticacheServiceUpdateActions_UpdateActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServiceUpdateActions_UpdateActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticacheServiceUpdateActions_UpdateActionsRef {
        DataElasticacheServiceUpdateActions_UpdateActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheServiceUpdateActions_UpdateActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_cluster_id` after provisioning.\n"]
    pub fn cache_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc = "Get a reference to the value of field `estimated_update_time` after provisioning.\n"]
    pub fn estimated_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `recommended_apply_by_date` after provisioning.\n"]
    pub fn recommended_apply_by_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recommended_apply_by_date", self.base))
    }

    #[doc = "Get a reference to the value of field `release_date` after provisioning.\n"]
    pub fn release_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_date", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_group_id` after provisioning.\n"]
    pub fn replication_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_update_name` after provisioning.\n"]
    pub fn service_update_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_update_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_update_severity` after provisioning.\n"]
    pub fn service_update_severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_update_severity", self.base))
    }

    #[doc = "Get a reference to the value of field `service_update_status` after provisioning.\n"]
    pub fn service_update_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_update_status", self.base))
    }

    #[doc = "Get a reference to the value of field `service_update_type` after provisioning.\n"]
    pub fn service_update_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_update_type", self.base))
    }

    #[doc = "Get a reference to the value of field `update_action_status` after provisioning.\n"]
    pub fn update_action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_action_status", self.base))
    }
}
