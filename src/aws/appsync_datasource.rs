#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppsyncDatasourceData_ {
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
    api_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb_config: std::option::Option<std::vec::Vec<AppsyncDatasource_DynamodbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch_config: std::option::Option<std::vec::Vec<AppsyncDatasource_ElasticsearchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_bridge_config: std::option::Option<std::vec::Vec<AppsyncDatasource_EventBridgeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_config: std::option::Option<std::vec::Vec<AppsyncDatasource_HttpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_config: std::option::Option<std::vec::Vec<AppsyncDatasource_LambdaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearchservice_config: std::option::Option<std::vec::Vec<AppsyncDatasource_OpensearchserviceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relational_database_config: std::option::Option<std::vec::Vec<AppsyncDatasource_RelationalDatabaseConfig>>,
    dynamic: AppsyncDatasourceDynamic,
}

struct AppsyncDatasource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppsyncDatasourceData_>,
}

#[derive(Clone)]
pub struct AppsyncDatasource(std::rc::Rc<AppsyncDatasource_>);

impl AppsyncDatasource {
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

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynamodb_config`.\n"]
    pub fn set_dynamodb_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_DynamodbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodb_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodb_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elasticsearch_config`.\n"]
    pub fn set_elasticsearch_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_ElasticsearchConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_bridge_config`.\n"]
    pub fn set_event_bridge_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_EventBridgeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_bridge_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_bridge_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_config`.\n"]
    pub fn set_http_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_HttpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_LambdaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `opensearchservice_config`.\n"]
    pub fn set_opensearchservice_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_OpensearchserviceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().opensearchservice_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.opensearchservice_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `relational_database_config`.\n"]
    pub fn set_relational_database_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_RelationalDatabaseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().relational_database_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.relational_database_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamodb_config` after provisioning.\n"]
    pub fn dynamodb_config(&self) -> terrars::ListRef<AppsyncDatasource_DynamodbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_config` after provisioning.\n"]
    pub fn elasticsearch_config(&self) -> terrars::ListRef<AppsyncDatasource_ElasticsearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_bridge_config` after provisioning.\n"]
    pub fn event_bridge_config(&self) -> terrars::ListRef<AppsyncDatasource_EventBridgeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_bridge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> terrars::ListRef<AppsyncDatasource_HttpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<AppsyncDatasource_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `opensearchservice_config` after provisioning.\n"]
    pub fn opensearchservice_config(&self) -> terrars::ListRef<AppsyncDatasource_OpensearchserviceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearchservice_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_config` after provisioning.\n"]
    pub fn relational_database_config(&self) -> terrars::ListRef<AppsyncDatasource_RelationalDatabaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relational_database_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppsyncDatasource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppsyncDatasource { }

impl terrars::ToListMappable for AppsyncDatasource {
    type O = terrars::ListRef<AppsyncDatasourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppsyncDatasource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appsync_datasource".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncDatasource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource {
    pub fn build(self, stack: &mut terrars::Stack) -> AppsyncDatasource {
        let out = AppsyncDatasource(std::rc::Rc::new(AppsyncDatasource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppsyncDatasourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                service_role_arn: core::default::Default::default(),
                type_: self.type_,
                dynamodb_config: core::default::Default::default(),
                elasticsearch_config: core::default::Default::default(),
                event_bridge_config: core::default::Default::default(),
                http_config: core::default::Default::default(),
                lambda_config: core::default::Default::default(),
                opensearchservice_config: core::default::Default::default(),
                relational_database_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncDatasourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppsyncDatasourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamodb_config` after provisioning.\n"]
    pub fn dynamodb_config(&self) -> terrars::ListRef<AppsyncDatasource_DynamodbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_config` after provisioning.\n"]
    pub fn elasticsearch_config(&self) -> terrars::ListRef<AppsyncDatasource_ElasticsearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_bridge_config` after provisioning.\n"]
    pub fn event_bridge_config(&self) -> terrars::ListRef<AppsyncDatasource_EventBridgeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_bridge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> terrars::ListRef<AppsyncDatasource_HttpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<AppsyncDatasource_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `opensearchservice_config` after provisioning.\n"]
    pub fn opensearchservice_config(&self) -> terrars::ListRef<AppsyncDatasource_OpensearchserviceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearchservice_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_config` after provisioning.\n"]
    pub fn relational_database_config(&self) -> terrars::ListRef<AppsyncDatasource_RelationalDatabaseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relational_database_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceDynamic {
    dynamodb_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_DynamodbConfig>>,
    elasticsearch_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_ElasticsearchConfig>>,
    event_bridge_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_EventBridgeConfig>>,
    http_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_HttpConfig>>,
    lambda_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_LambdaConfig>>,
    opensearchservice_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_OpensearchserviceConfig>>,
    relational_database_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasource_RelationalDatabaseConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_table_ttl: std::option::Option<terrars::PrimField<f64>>,
    delta_sync_table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delta_sync_table_ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl AppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
    #[doc = "Set the field `base_table_ttl`.\n"]
    pub fn set_base_table_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base_table_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delta_sync_table_ttl`.\n"]
    pub fn set_delta_sync_table_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.delta_sync_table_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_DynamodbConfig_DeltaSyncConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
    #[doc = ""]
    pub delta_sync_table_name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
    pub fn build(self) -> AppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
        AppsyncDatasource_DynamodbConfig_DeltaSyncConfig {
            base_table_ttl: core::default::Default::default(),
            delta_sync_table_name: self.delta_sync_table_name,
            delta_sync_table_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_DynamodbConfig_DeltaSyncConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_DynamodbConfig_DeltaSyncConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasource_DynamodbConfig_DeltaSyncConfigRef {
        AppsyncDatasource_DynamodbConfig_DeltaSyncConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_DynamodbConfig_DeltaSyncConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_table_ttl` after provisioning.\n"]
    pub fn base_table_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_table_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `delta_sync_table_name` after provisioning.\n"]
    pub fn delta_sync_table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delta_sync_table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `delta_sync_table_ttl` after provisioning.\n"]
    pub fn delta_sync_table_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delta_sync_table_ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasource_DynamodbConfigDynamic {
    delta_sync_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasource_DynamodbConfig_DeltaSyncConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasource_DynamodbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_caller_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delta_sync_config: std::option::Option<std::vec::Vec<AppsyncDatasource_DynamodbConfig_DeltaSyncConfig>>,
    dynamic: AppsyncDatasource_DynamodbConfigDynamic,
}

impl AppsyncDatasource_DynamodbConfig {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_caller_credentials`.\n"]
    pub fn set_use_caller_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_caller_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versioned`.\n"]
    pub fn set_versioned(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.versioned = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delta_sync_config`.\n"]
    pub fn set_delta_sync_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_DynamodbConfig_DeltaSyncConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delta_sync_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delta_sync_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_DynamodbConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_DynamodbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_DynamodbConfig {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_DynamodbConfig {
    pub fn build(self) -> AppsyncDatasource_DynamodbConfig {
        AppsyncDatasource_DynamodbConfig {
            region: core::default::Default::default(),
            table_name: self.table_name,
            use_caller_credentials: core::default::Default::default(),
            versioned: core::default::Default::default(),
            delta_sync_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_DynamodbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_DynamodbConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_DynamodbConfigRef {
        AppsyncDatasource_DynamodbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_DynamodbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `use_caller_credentials` after provisioning.\n"]
    pub fn use_caller_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_caller_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `versioned` after provisioning.\n"]
    pub fn versioned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned", self.base))
    }

    #[doc = "Get a reference to the value of field `delta_sync_config` after provisioning.\n"]
    pub fn delta_sync_config(&self) -> terrars::ListRef<AppsyncDatasource_DynamodbConfig_DeltaSyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delta_sync_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasource_ElasticsearchConfig {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasource_ElasticsearchConfig {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_ElasticsearchConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_ElasticsearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_ElasticsearchConfig {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_ElasticsearchConfig {
    pub fn build(self) -> AppsyncDatasource_ElasticsearchConfig {
        AppsyncDatasource_ElasticsearchConfig {
            endpoint: self.endpoint,
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_ElasticsearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_ElasticsearchConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_ElasticsearchConfigRef {
        AppsyncDatasource_ElasticsearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_ElasticsearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasource_EventBridgeConfig {
    event_bus_arn: terrars::PrimField<std::string::String>,
}

impl AppsyncDatasource_EventBridgeConfig { }

impl terrars::ToListMappable for AppsyncDatasource_EventBridgeConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_EventBridgeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_EventBridgeConfig {
    #[doc = ""]
    pub event_bus_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_EventBridgeConfig {
    pub fn build(self) -> AppsyncDatasource_EventBridgeConfig {
        AppsyncDatasource_EventBridgeConfig { event_bus_arn: self.event_bus_arn }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_EventBridgeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_EventBridgeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_EventBridgeConfigRef {
        AppsyncDatasource_EventBridgeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_EventBridgeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_bus_arn` after provisioning.\n"]
    pub fn event_bus_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_bus_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {
    #[doc = "Set the field `signing_region`.\n"]
    pub fn set_signing_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signing_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signing_service_name`.\n"]
    pub fn set_signing_service_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.signing_service_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {}

impl BuildAppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {
    pub fn build(self) -> AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {
        AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig {
            signing_region: core::default::Default::default(),
            signing_service_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfigRef {
        AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `signing_region` after provisioning.\n"]
    pub fn signing_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_region", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_service_name` after provisioning.\n"]
    pub fn signing_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasource_HttpConfig_AuthorizationConfigDynamic {
    aws_iam_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasource_HttpConfig_AuthorizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_config: std::option::Option<std::vec::Vec<AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig>>,
    dynamic: AppsyncDatasource_HttpConfig_AuthorizationConfigDynamic,
}

impl AppsyncDatasource_HttpConfig_AuthorizationConfig {
    #[doc = "Set the field `authorization_type`.\n"]
    pub fn set_authorization_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_iam_config`.\n"]
    pub fn set_aws_iam_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_iam_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iam_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_HttpConfig_AuthorizationConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_HttpConfig_AuthorizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_HttpConfig_AuthorizationConfig {}

impl BuildAppsyncDatasource_HttpConfig_AuthorizationConfig {
    pub fn build(self) -> AppsyncDatasource_HttpConfig_AuthorizationConfig {
        AppsyncDatasource_HttpConfig_AuthorizationConfig {
            authorization_type: core::default::Default::default(),
            aws_iam_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_HttpConfig_AuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_HttpConfig_AuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasource_HttpConfig_AuthorizationConfigRef {
        AppsyncDatasource_HttpConfig_AuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_HttpConfig_AuthorizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_iam_config` after provisioning.\n"]
    pub fn aws_iam_config(&self) -> terrars::ListRef<AppsyncDatasource_HttpConfig_AuthorizationConfig_AwsIamConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iam_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasource_HttpConfigDynamic {
    authorization_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasource_HttpConfig_AuthorizationConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasource_HttpConfig {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std::option::Option<std::vec::Vec<AppsyncDatasource_HttpConfig_AuthorizationConfig>>,
    dynamic: AppsyncDatasource_HttpConfigDynamic,
}

impl AppsyncDatasource_HttpConfig {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasource_HttpConfig_AuthorizationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_HttpConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_HttpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_HttpConfig {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_HttpConfig {
    pub fn build(self) -> AppsyncDatasource_HttpConfig {
        AppsyncDatasource_HttpConfig {
            endpoint: self.endpoint,
            authorization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_HttpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_HttpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_HttpConfigRef {
        AppsyncDatasource_HttpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_HttpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(&self) -> terrars::ListRef<AppsyncDatasource_HttpConfig_AuthorizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasource_LambdaConfig {
    function_arn: terrars::PrimField<std::string::String>,
}

impl AppsyncDatasource_LambdaConfig { }

impl terrars::ToListMappable for AppsyncDatasource_LambdaConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_LambdaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_LambdaConfig {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_LambdaConfig {
    pub fn build(self) -> AppsyncDatasource_LambdaConfig {
        AppsyncDatasource_LambdaConfig { function_arn: self.function_arn }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_LambdaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_LambdaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_LambdaConfigRef {
        AppsyncDatasource_LambdaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_LambdaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasource_OpensearchserviceConfig {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasource_OpensearchserviceConfig {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_OpensearchserviceConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_OpensearchserviceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_OpensearchserviceConfig {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_OpensearchserviceConfig {
    pub fn build(self) -> AppsyncDatasource_OpensearchserviceConfig {
        AppsyncDatasource_OpensearchserviceConfig {
            endpoint: self.endpoint,
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_OpensearchserviceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_OpensearchserviceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_OpensearchserviceConfigRef {
        AppsyncDatasource_OpensearchserviceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_OpensearchserviceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
    aws_secret_store_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    db_cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
    #[doc = ""]
    pub aws_secret_store_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub db_cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
    pub fn build(self) -> AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
        AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig {
            aws_secret_store_arn: self.aws_secret_store_arn,
            database_name: core::default::Default::default(),
            db_cluster_identifier: self.db_cluster_identifier,
            region: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfigRef {
        AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_secret_store_arn` after provisioning.\n"]
    pub fn aws_secret_store_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_secret_store_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `db_cluster_identifier` after provisioning.\n"]
    pub fn db_cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasource_RelationalDatabaseConfigDynamic {
    http_endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasource_RelationalDatabaseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint_config: std
    ::option
    ::Option<std::vec::Vec<AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig>>,
    dynamic: AppsyncDatasource_RelationalDatabaseConfigDynamic,
}

impl AppsyncDatasource_RelationalDatabaseConfig {
    #[doc = "Set the field `source_type`.\n"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_endpoint_config`.\n"]
    pub fn set_http_endpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_endpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_endpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasource_RelationalDatabaseConfig {
    type O = terrars::BlockAssignable<AppsyncDatasource_RelationalDatabaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasource_RelationalDatabaseConfig {}

impl BuildAppsyncDatasource_RelationalDatabaseConfig {
    pub fn build(self) -> AppsyncDatasource_RelationalDatabaseConfig {
        AppsyncDatasource_RelationalDatabaseConfig {
            source_type: core::default::Default::default(),
            http_endpoint_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncDatasource_RelationalDatabaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasource_RelationalDatabaseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasource_RelationalDatabaseConfigRef {
        AppsyncDatasource_RelationalDatabaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasource_RelationalDatabaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `http_endpoint_config` after provisioning.\n"]
    pub fn http_endpoint_config(
        &self,
    ) -> terrars::ListRef<AppsyncDatasource_RelationalDatabaseConfig_HttpEndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint_config", self.base))
    }
}
