use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServiceDiscoveryServiceData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    namespace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataServiceDiscoveryService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataServiceDiscoveryServiceData>,
}

#[derive(Clone)]
pub struct DataServiceDiscoveryService(std::rc::Rc<DataServiceDiscoveryService_>);

impl DataServiceDiscoveryService {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<DataServiceDiscoveryServiceDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> terrars::ListRef<DataServiceDiscoveryServiceHealthCheckConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(
        &self,
    ) -> terrars::ListRef<DataServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for DataServiceDiscoveryService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataServiceDiscoveryService { }

impl terrars::ToListMappable for DataServiceDiscoveryService {
    type O = terrars::ListRef<DataServiceDiscoveryServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataServiceDiscoveryService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_service_discovery_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServiceDiscoveryService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace_id: terrars::PrimField<std::string::String>,
}

impl BuildDataServiceDiscoveryService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataServiceDiscoveryService {
        let out = DataServiceDiscoveryService(std::rc::Rc::new(DataServiceDiscoveryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataServiceDiscoveryServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                namespace_id: self.namespace_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServiceDiscoveryServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServiceDiscoveryServiceRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<DataServiceDiscoveryServiceDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> terrars::ListRef<DataServiceDiscoveryServiceHealthCheckConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(
        &self,
    ) -> terrars::ListRef<DataServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    type O = terrars::BlockAssignable<DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {}

impl BuildDataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    pub fn build(self) -> DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
        DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
        DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceDnsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_records: std::option::Option<terrars::ListField<DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServiceDiscoveryServiceDnsConfigEl {
    #[doc = "Set the field `dns_records`.\n"]
    pub fn set_dns_records(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    ) -> Self {
        self.dns_records = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_id`.\n"]
    pub fn set_namespace_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServiceDiscoveryServiceDnsConfigEl {
    type O = terrars::BlockAssignable<DataServiceDiscoveryServiceDnsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceDnsConfigEl {}

impl BuildDataServiceDiscoveryServiceDnsConfigEl {
    pub fn build(self) -> DataServiceDiscoveryServiceDnsConfigEl {
        DataServiceDiscoveryServiceDnsConfigEl {
            dns_records: core::default::Default::default(),
            namespace_id: core::default::Default::default(),
            routing_policy: core::default::Default::default(),
        }
    }
}

pub struct DataServiceDiscoveryServiceDnsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryServiceDnsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataServiceDiscoveryServiceDnsConfigElRef {
        DataServiceDiscoveryServiceDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceDnsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> terrars::ListRef<DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_records", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceHealthCheckConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServiceDiscoveryServiceHealthCheckConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_path`.\n"]
    pub fn set_resource_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServiceDiscoveryServiceHealthCheckConfigEl {
    type O = terrars::BlockAssignable<DataServiceDiscoveryServiceHealthCheckConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceHealthCheckConfigEl {}

impl BuildDataServiceDiscoveryServiceHealthCheckConfigEl {
    pub fn build(self) -> DataServiceDiscoveryServiceHealthCheckConfigEl {
        DataServiceDiscoveryServiceHealthCheckConfigEl {
            failure_threshold: core::default::Default::default(),
            resource_path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServiceDiscoveryServiceHealthCheckConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryServiceHealthCheckConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServiceDiscoveryServiceHealthCheckConfigElRef {
        DataServiceDiscoveryServiceHealthCheckConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceHealthCheckConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_path` after provisioning.\n"]
    pub fn resource_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_path", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    type O = terrars::BlockAssignable<DataServiceDiscoveryServiceHealthCheckCustomConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceHealthCheckCustomConfigEl {}

impl BuildDataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    pub fn build(self) -> DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
        DataServiceDiscoveryServiceHealthCheckCustomConfigEl { failure_threshold: core::default::Default::default() }
    }
}

pub struct DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
        DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }
}
