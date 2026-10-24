use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppsyncDatasourceData {
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
    service_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb_config: std::option::Option<std::vec::Vec<AppsyncDatasourceDynamodbConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch_config: std::option::Option<std::vec::Vec<AppsyncDatasourceElasticsearchConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_config: std::option::Option<std::vec::Vec<AppsyncDatasourceHttpConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_config: std::option::Option<std::vec::Vec<AppsyncDatasourceLambdaConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relational_database_config: std::option::Option<std::vec::Vec<AppsyncDatasourceRelationalDatabaseConfigEl>>,
    dynamic: AppsyncDatasourceDynamic,
}

struct AppsyncDatasource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppsyncDatasourceData>,
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

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynamodb_config`.\n"]
    pub fn set_dynamodb_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceDynamodbConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceElasticsearchConfigEl>>,
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

    #[doc = "Set the field `http_config`.\n"]
    pub fn set_http_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceHttpConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceLambdaConfigEl>>,
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

    #[doc = "Set the field `relational_database_config`.\n"]
    pub fn set_relational_database_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigEl>>,
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

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamodb_config` after provisioning.\n"]
    pub fn dynamodb_config(&self) -> terrars::ListRef<AppsyncDatasourceDynamodbConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_config` after provisioning.\n"]
    pub fn elasticsearch_config(&self) -> terrars::ListRef<AppsyncDatasourceElasticsearchConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> terrars::ListRef<AppsyncDatasourceHttpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<AppsyncDatasourceLambdaConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_config` after provisioning.\n"]
    pub fn relational_database_config(&self) -> terrars::ListRef<AppsyncDatasourceRelationalDatabaseConfigElRef> {
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
            data: std::cell::RefCell::new(AppsyncDatasourceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                service_role_arn: core::default::Default::default(),
                type_: self.type_,
                dynamodb_config: core::default::Default::default(),
                elasticsearch_config: core::default::Default::default(),
                http_config: core::default::Default::default(),
                lambda_config: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamodb_config` after provisioning.\n"]
    pub fn dynamodb_config(&self) -> terrars::ListRef<AppsyncDatasourceDynamodbConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_config` after provisioning.\n"]
    pub fn elasticsearch_config(&self) -> terrars::ListRef<AppsyncDatasourceElasticsearchConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> terrars::ListRef<AppsyncDatasourceHttpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<AppsyncDatasourceLambdaConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_config` after provisioning.\n"]
    pub fn relational_database_config(&self) -> terrars::ListRef<AppsyncDatasourceRelationalDatabaseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relational_database_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_table_ttl: std::option::Option<terrars::PrimField<f64>>,
    delta_sync_table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delta_sync_table_ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
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

impl terrars::ToListMappable for AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    #[doc = ""]
    pub delta_sync_table_name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
    pub fn build(self) -> AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
        AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl {
            base_table_ttl: core::default::Default::default(),
            delta_sync_table_name: self.delta_sync_table_name,
            delta_sync_table_ttl: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
        AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef {
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
struct AppsyncDatasourceDynamodbConfigElDynamic {
    delta_sync_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceDynamodbConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_caller_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delta_sync_config: std::option::Option<std::vec::Vec<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>>,
    dynamic: AppsyncDatasourceDynamodbConfigElDynamic,
}

impl AppsyncDatasourceDynamodbConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigEl>>,
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

impl terrars::ToListMappable for AppsyncDatasourceDynamodbConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceDynamodbConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceDynamodbConfigEl {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasourceDynamodbConfigEl {
    pub fn build(self) -> AppsyncDatasourceDynamodbConfigEl {
        AppsyncDatasourceDynamodbConfigEl {
            region: core::default::Default::default(),
            table_name: self.table_name,
            use_caller_credentials: core::default::Default::default(),
            versioned: core::default::Default::default(),
            delta_sync_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceDynamodbConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceDynamodbConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasourceDynamodbConfigElRef {
        AppsyncDatasourceDynamodbConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceDynamodbConfigElRef {
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
    pub fn delta_sync_config(&self) -> terrars::ListRef<AppsyncDatasourceDynamodbConfigElDeltaSyncConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delta_sync_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceElasticsearchConfigEl {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasourceElasticsearchConfigEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncDatasourceElasticsearchConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceElasticsearchConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceElasticsearchConfigEl {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasourceElasticsearchConfigEl {
    pub fn build(self) -> AppsyncDatasourceElasticsearchConfigEl {
        AppsyncDatasourceElasticsearchConfigEl {
            endpoint: self.endpoint,
            region: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceElasticsearchConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceElasticsearchConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasourceElasticsearchConfigElRef {
        AppsyncDatasourceElasticsearchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceElasticsearchConfigElRef {
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
pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
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

impl terrars::ToListMappable for AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {}

impl BuildAppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
    pub fn build(self) -> AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
        AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl {
            signing_region: core::default::Default::default(),
            signing_service_name: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
        AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef {
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
struct AppsyncDatasourceHttpConfigElAuthorizationConfigElDynamic {
    aws_iam_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_config: std::option::Option<std::vec::Vec<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>>,
    dynamic: AppsyncDatasourceHttpConfigElAuthorizationConfigElDynamic,
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
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
                    ::Into<
                        terrars::BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigEl>,
                    >,
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

impl terrars::ToListMappable for AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceHttpConfigElAuthorizationConfigEl {}

impl BuildAppsyncDatasourceHttpConfigElAuthorizationConfigEl {
    pub fn build(self) -> AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
        AppsyncDatasourceHttpConfigElAuthorizationConfigEl {
            authorization_type: core::default::Default::default(),
            aws_iam_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
        AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceHttpConfigElAuthorizationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_iam_config` after provisioning.\n"]
    pub fn aws_iam_config(
        &self,
    ) -> terrars::ListRef<AppsyncDatasourceHttpConfigElAuthorizationConfigElAwsIamConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iam_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceHttpConfigElDynamic {
    authorization_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceHttpConfigEl {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std::option::Option<std::vec::Vec<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>>,
    dynamic: AppsyncDatasourceHttpConfigElDynamic,
}

impl AppsyncDatasourceHttpConfigEl {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncDatasourceHttpConfigElAuthorizationConfigEl>>,
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

impl terrars::ToListMappable for AppsyncDatasourceHttpConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceHttpConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceHttpConfigEl {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasourceHttpConfigEl {
    pub fn build(self) -> AppsyncDatasourceHttpConfigEl {
        AppsyncDatasourceHttpConfigEl {
            endpoint: self.endpoint,
            authorization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceHttpConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceHttpConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasourceHttpConfigElRef {
        AppsyncDatasourceHttpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceHttpConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(&self) -> terrars::ListRef<AppsyncDatasourceHttpConfigElAuthorizationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceLambdaConfigEl {
    function_arn: terrars::PrimField<std::string::String>,
}

impl AppsyncDatasourceLambdaConfigEl { }

impl terrars::ToListMappable for AppsyncDatasourceLambdaConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceLambdaConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceLambdaConfigEl {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasourceLambdaConfigEl {
    pub fn build(self) -> AppsyncDatasourceLambdaConfigEl {
        AppsyncDatasourceLambdaConfigEl { function_arn: self.function_arn }
    }
}

pub struct AppsyncDatasourceLambdaConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceLambdaConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncDatasourceLambdaConfigElRef {
        AppsyncDatasourceLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceLambdaConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    aws_secret_store_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    db_cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
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

impl terrars::ToListMappable for AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    #[doc = ""]
    pub aws_secret_store_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub db_cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
    pub fn build(self) -> AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
        AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl {
            aws_secret_store_arn: self.aws_secret_store_arn,
            database_name: core::default::Default::default(),
            db_cluster_identifier: self.db_cluster_identifier,
            region: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
        AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef {
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
struct AppsyncDatasourceRelationalDatabaseConfigElDynamic {
    http_endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncDatasourceRelationalDatabaseConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint_config: std
    ::option
    ::Option<std::vec::Vec<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>>,
    dynamic: AppsyncDatasourceRelationalDatabaseConfigElDynamic,
}

impl AppsyncDatasourceRelationalDatabaseConfigEl {
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
                    ::Into<terrars::BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigEl>>,
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

impl terrars::ToListMappable for AppsyncDatasourceRelationalDatabaseConfigEl {
    type O = terrars::BlockAssignable<AppsyncDatasourceRelationalDatabaseConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncDatasourceRelationalDatabaseConfigEl {}

impl BuildAppsyncDatasourceRelationalDatabaseConfigEl {
    pub fn build(self) -> AppsyncDatasourceRelationalDatabaseConfigEl {
        AppsyncDatasourceRelationalDatabaseConfigEl {
            source_type: core::default::Default::default(),
            http_endpoint_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppsyncDatasourceRelationalDatabaseConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncDatasourceRelationalDatabaseConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncDatasourceRelationalDatabaseConfigElRef {
        AppsyncDatasourceRelationalDatabaseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncDatasourceRelationalDatabaseConfigElRef {
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
    ) -> terrars::ListRef<AppsyncDatasourceRelationalDatabaseConfigElHttpEndpointConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncDatasourceDynamic {
    dynamodb_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasourceDynamodbConfigEl>>,
    elasticsearch_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasourceElasticsearchConfigEl>>,
    http_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasourceHttpConfigEl>>,
    lambda_config: std::option::Option<terrars::DynamicBlock<AppsyncDatasourceLambdaConfigEl>>,
    relational_database_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncDatasourceRelationalDatabaseConfigEl>>,
}
