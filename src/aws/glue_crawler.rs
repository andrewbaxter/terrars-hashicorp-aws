#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCrawlerData_ {
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
    classifiers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_target: std::option::Option<std::vec::Vec<GlueCrawler_CatalogTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delta_target: std::option::Option<std::vec::Vec<GlueCrawler_DeltaTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb_target: std::option::Option<std::vec::Vec<GlueCrawler_DynamodbTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hudi_target: std::option::Option<std::vec::Vec<GlueCrawler_HudiTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_target: std::option::Option<std::vec::Vec<GlueCrawler_IcebergTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jdbc_target: std::option::Option<std::vec::Vec<GlueCrawler_JdbcTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lake_formation_configuration: std::option::Option<std::vec::Vec<GlueCrawler_LakeFormationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lineage_configuration: std::option::Option<std::vec::Vec<GlueCrawler_LineageConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_target: std::option::Option<std::vec::Vec<GlueCrawler_MongodbTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recrawl_policy: std::option::Option<std::vec::Vec<GlueCrawler_RecrawlPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_target: std::option::Option<std::vec::Vec<GlueCrawler_S3Target>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_change_policy: std::option::Option<std::vec::Vec<GlueCrawler_SchemaChangePolicy>>,
    dynamic: GlueCrawlerDynamic,
}

struct GlueCrawler_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueCrawlerData_>,
}

#[derive(Clone)]
pub struct GlueCrawler(std::rc::Rc<GlueCrawler_>);

impl GlueCrawler {
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

    #[doc = "Set the field `classifiers`.\n"]
    pub fn set_classifiers(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().classifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().configuration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().security_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_prefix`.\n"]
    pub fn set_table_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().table_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `catalog_target`.\n"]
    pub fn set_catalog_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_CatalogTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().catalog_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.catalog_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delta_target`.\n"]
    pub fn set_delta_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_DeltaTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delta_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delta_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamodb_target`.\n"]
    pub fn set_dynamodb_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_DynamodbTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodb_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodb_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hudi_target`.\n"]
    pub fn set_hudi_target(self, v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_HudiTarget>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hudi_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hudi_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iceberg_target`.\n"]
    pub fn set_iceberg_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_IcebergTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iceberg_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iceberg_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jdbc_target`.\n"]
    pub fn set_jdbc_target(self, v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_JdbcTarget>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jdbc_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jdbc_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lake_formation_configuration`.\n"]
    pub fn set_lake_formation_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_LakeFormationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lake_formation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lake_formation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lineage_configuration`.\n"]
    pub fn set_lineage_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_LineageConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lineage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lineage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mongodb_target`.\n"]
    pub fn set_mongodb_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_MongodbTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recrawl_policy`.\n"]
    pub fn set_recrawl_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_RecrawlPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recrawl_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recrawl_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_target`.\n"]
    pub fn set_s3_target(self, v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_S3Target>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_change_policy`.\n"]
    pub fn set_schema_change_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCrawler_SchemaChangePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_change_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_change_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `classifiers` after provisioning.\n"]
    pub fn classifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classifiers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_target` after provisioning.\n"]
    pub fn catalog_target(&self) -> terrars::ListRef<GlueCrawler_CatalogTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delta_target` after provisioning.\n"]
    pub fn delta_target(&self) -> terrars::ListRef<GlueCrawler_DeltaTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delta_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamodb_target` after provisioning.\n"]
    pub fn dynamodb_target(&self) -> terrars::ListRef<GlueCrawler_DynamodbTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hudi_target` after provisioning.\n"]
    pub fn hudi_target(&self) -> terrars::ListRef<GlueCrawler_HudiTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hudi_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iceberg_target` after provisioning.\n"]
    pub fn iceberg_target(&self) -> terrars::ListRef<GlueCrawler_IcebergTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jdbc_target` after provisioning.\n"]
    pub fn jdbc_target(&self) -> terrars::ListRef<GlueCrawler_JdbcTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jdbc_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lake_formation_configuration` after provisioning.\n"]
    pub fn lake_formation_configuration(&self) -> terrars::ListRef<GlueCrawler_LakeFormationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lake_formation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lineage_configuration` after provisioning.\n"]
    pub fn lineage_configuration(&self) -> terrars::ListRef<GlueCrawler_LineageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lineage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_target` after provisioning.\n"]
    pub fn mongodb_target(&self) -> terrars::ListRef<GlueCrawler_MongodbTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recrawl_policy` after provisioning.\n"]
    pub fn recrawl_policy(&self) -> terrars::ListRef<GlueCrawler_RecrawlPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recrawl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_target` after provisioning.\n"]
    pub fn s3_target(&self) -> terrars::ListRef<GlueCrawler_S3TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_change_policy` after provisioning.\n"]
    pub fn schema_change_policy(&self) -> terrars::ListRef<GlueCrawler_SchemaChangePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_change_policy", self.extract_ref()))
    }
}

impl terrars::Referable for GlueCrawler {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueCrawler { }

impl terrars::ToListMappable for GlueCrawler {
    type O = terrars::ListRef<GlueCrawlerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueCrawler_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_crawler".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCrawler {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildGlueCrawler {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueCrawler {
        let out = GlueCrawler(std::rc::Rc::new(GlueCrawler_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueCrawlerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                classifiers: core::default::Default::default(),
                configuration: core::default::Default::default(),
                database_name: self.database_name,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role: self.role,
                schedule: core::default::Default::default(),
                security_configuration: core::default::Default::default(),
                table_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                catalog_target: core::default::Default::default(),
                delta_target: core::default::Default::default(),
                dynamodb_target: core::default::Default::default(),
                hudi_target: core::default::Default::default(),
                iceberg_target: core::default::Default::default(),
                jdbc_target: core::default::Default::default(),
                lake_formation_configuration: core::default::Default::default(),
                lineage_configuration: core::default::Default::default(),
                mongodb_target: core::default::Default::default(),
                recrawl_policy: core::default::Default::default(),
                s3_target: core::default::Default::default(),
                schema_change_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCrawlerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawlerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueCrawlerRef {
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

    #[doc = "Get a reference to the value of field `classifiers` after provisioning.\n"]
    pub fn classifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classifiers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_target` after provisioning.\n"]
    pub fn catalog_target(&self) -> terrars::ListRef<GlueCrawler_CatalogTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delta_target` after provisioning.\n"]
    pub fn delta_target(&self) -> terrars::ListRef<GlueCrawler_DeltaTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delta_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamodb_target` after provisioning.\n"]
    pub fn dynamodb_target(&self) -> terrars::ListRef<GlueCrawler_DynamodbTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hudi_target` after provisioning.\n"]
    pub fn hudi_target(&self) -> terrars::ListRef<GlueCrawler_HudiTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hudi_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iceberg_target` after provisioning.\n"]
    pub fn iceberg_target(&self) -> terrars::ListRef<GlueCrawler_IcebergTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jdbc_target` after provisioning.\n"]
    pub fn jdbc_target(&self) -> terrars::ListRef<GlueCrawler_JdbcTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jdbc_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lake_formation_configuration` after provisioning.\n"]
    pub fn lake_formation_configuration(&self) -> terrars::ListRef<GlueCrawler_LakeFormationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lake_formation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lineage_configuration` after provisioning.\n"]
    pub fn lineage_configuration(&self) -> terrars::ListRef<GlueCrawler_LineageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lineage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_target` after provisioning.\n"]
    pub fn mongodb_target(&self) -> terrars::ListRef<GlueCrawler_MongodbTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recrawl_policy` after provisioning.\n"]
    pub fn recrawl_policy(&self) -> terrars::ListRef<GlueCrawler_RecrawlPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recrawl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_target` after provisioning.\n"]
    pub fn s3_target(&self) -> terrars::ListRef<GlueCrawler_S3TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_change_policy` after provisioning.\n"]
    pub fn schema_change_policy(&self) -> terrars::ListRef<GlueCrawler_SchemaChangePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_change_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueCrawlerDynamic {
    catalog_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_CatalogTarget>>,
    delta_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_DeltaTarget>>,
    dynamodb_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_DynamodbTarget>>,
    hudi_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_HudiTarget>>,
    iceberg_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_IcebergTarget>>,
    jdbc_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_JdbcTarget>>,
    lake_formation_configuration: std::option::Option<terrars::DynamicBlock<GlueCrawler_LakeFormationConfiguration>>,
    lineage_configuration: std::option::Option<terrars::DynamicBlock<GlueCrawler_LineageConfiguration>>,
    mongodb_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_MongodbTarget>>,
    recrawl_policy: std::option::Option<terrars::DynamicBlock<GlueCrawler_RecrawlPolicy>>,
    s3_target: std::option::Option<terrars::DynamicBlock<GlueCrawler_S3Target>>,
    schema_change_policy: std::option::Option<terrars::DynamicBlock<GlueCrawler_SchemaChangePolicy>>,
}

#[derive(Serialize)]
pub struct GlueCrawler_CatalogTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dlq_event_queue_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_queue_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    tables: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GlueCrawler_CatalogTarget {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dlq_event_queue_arn`.\n"]
    pub fn set_dlq_event_queue_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dlq_event_queue_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_queue_arn`.\n"]
    pub fn set_event_queue_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_queue_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_CatalogTarget {
    type O = terrars::BlockAssignable<GlueCrawler_CatalogTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_CatalogTarget {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub tables: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGlueCrawler_CatalogTarget {
    pub fn build(self) -> GlueCrawler_CatalogTarget {
        GlueCrawler_CatalogTarget {
            connection_name: core::default::Default::default(),
            database_name: self.database_name,
            dlq_event_queue_arn: core::default::Default::default(),
            event_queue_arn: core::default::Default::default(),
            tables: self.tables,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_CatalogTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_CatalogTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_CatalogTargetRef {
        GlueCrawler_CatalogTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_CatalogTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dlq_event_queue_arn` after provisioning.\n"]
    pub fn dlq_event_queue_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dlq_event_queue_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `event_queue_arn` after provisioning.\n"]
    pub fn event_queue_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_queue_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_DeltaTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_native_delta_table: std::option::Option<terrars::PrimField<bool>>,
    delta_tables: terrars::SetField<terrars::PrimField<std::string::String>>,
    write_manifest: terrars::PrimField<bool>,
}

impl GlueCrawler_DeltaTarget {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_native_delta_table`.\n"]
    pub fn set_create_native_delta_table(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.create_native_delta_table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_DeltaTarget {
    type O = terrars::BlockAssignable<GlueCrawler_DeltaTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_DeltaTarget {
    #[doc = ""]
    pub delta_tables: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub write_manifest: terrars::PrimField<bool>,
}

impl BuildGlueCrawler_DeltaTarget {
    pub fn build(self) -> GlueCrawler_DeltaTarget {
        GlueCrawler_DeltaTarget {
            connection_name: core::default::Default::default(),
            create_native_delta_table: core::default::Default::default(),
            delta_tables: self.delta_tables,
            write_manifest: self.write_manifest,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_DeltaTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_DeltaTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_DeltaTargetRef {
        GlueCrawler_DeltaTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_DeltaTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `create_native_delta_table` after provisioning.\n"]
    pub fn create_native_delta_table(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_native_delta_table", self.base))
    }

    #[doc = "Get a reference to the value of field `delta_tables` after provisioning.\n"]
    pub fn delta_tables(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delta_tables", self.base))
    }

    #[doc = "Get a reference to the value of field `write_manifest` after provisioning.\n"]
    pub fn write_manifest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_manifest", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_DynamodbTarget {
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueCrawler_DynamodbTarget {
    #[doc = "Set the field `scan_all`.\n"]
    pub fn set_scan_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.scan_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_rate`.\n"]
    pub fn set_scan_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scan_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_DynamodbTarget {
    type O = terrars::BlockAssignable<GlueCrawler_DynamodbTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_DynamodbTarget {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildGlueCrawler_DynamodbTarget {
    pub fn build(self) -> GlueCrawler_DynamodbTarget {
        GlueCrawler_DynamodbTarget {
            path: self.path,
            scan_all: core::default::Default::default(),
            scan_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_DynamodbTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_DynamodbTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_DynamodbTargetRef {
        GlueCrawler_DynamodbTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_DynamodbTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_all` after provisioning.\n"]
    pub fn scan_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_all", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_rate` after provisioning.\n"]
    pub fn scan_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_HudiTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    maximum_traversal_depth: terrars::PrimField<f64>,
    paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl GlueCrawler_HudiTarget {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_HudiTarget {
    type O = terrars::BlockAssignable<GlueCrawler_HudiTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_HudiTarget {
    #[doc = ""]
    pub maximum_traversal_depth: terrars::PrimField<f64>,
    #[doc = ""]
    pub paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildGlueCrawler_HudiTarget {
    pub fn build(self) -> GlueCrawler_HudiTarget {
        GlueCrawler_HudiTarget {
            connection_name: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            maximum_traversal_depth: self.maximum_traversal_depth,
            paths: self.paths,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_HudiTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_HudiTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_HudiTargetRef {
        GlueCrawler_HudiTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_HudiTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_traversal_depth` after provisioning.\n"]
    pub fn maximum_traversal_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_traversal_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_IcebergTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    maximum_traversal_depth: terrars::PrimField<f64>,
    paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl GlueCrawler_IcebergTarget {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_IcebergTarget {
    type O = terrars::BlockAssignable<GlueCrawler_IcebergTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_IcebergTarget {
    #[doc = ""]
    pub maximum_traversal_depth: terrars::PrimField<f64>,
    #[doc = ""]
    pub paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildGlueCrawler_IcebergTarget {
    pub fn build(self) -> GlueCrawler_IcebergTarget {
        GlueCrawler_IcebergTarget {
            connection_name: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            maximum_traversal_depth: self.maximum_traversal_depth,
            paths: self.paths,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_IcebergTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_IcebergTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_IcebergTargetRef {
        GlueCrawler_IcebergTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_IcebergTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_traversal_depth` after provisioning.\n"]
    pub fn maximum_traversal_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_traversal_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_JdbcTarget {
    connection_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_additional_metadata: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    path: terrars::PrimField<std::string::String>,
}

impl GlueCrawler_JdbcTarget {
    #[doc = "Set the field `enable_additional_metadata`.\n"]
    pub fn set_enable_additional_metadata(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enable_additional_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_JdbcTarget {
    type O = terrars::BlockAssignable<GlueCrawler_JdbcTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_JdbcTarget {
    #[doc = ""]
    pub connection_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildGlueCrawler_JdbcTarget {
    pub fn build(self) -> GlueCrawler_JdbcTarget {
        GlueCrawler_JdbcTarget {
            connection_name: self.connection_name,
            enable_additional_metadata: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_JdbcTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_JdbcTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_JdbcTargetRef {
        GlueCrawler_JdbcTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_JdbcTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_additional_metadata` after provisioning.\n"]
    pub fn enable_additional_metadata(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enable_additional_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_LakeFormationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_lake_formation_credentials: std::option::Option<terrars::PrimField<bool>>,
}

impl GlueCrawler_LakeFormationConfiguration {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_lake_formation_credentials`.\n"]
    pub fn set_use_lake_formation_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_lake_formation_credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_LakeFormationConfiguration {
    type O = terrars::BlockAssignable<GlueCrawler_LakeFormationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_LakeFormationConfiguration {}

impl BuildGlueCrawler_LakeFormationConfiguration {
    pub fn build(self) -> GlueCrawler_LakeFormationConfiguration {
        GlueCrawler_LakeFormationConfiguration {
            account_id: core::default::Default::default(),
            use_lake_formation_credentials: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_LakeFormationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_LakeFormationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_LakeFormationConfigurationRef {
        GlueCrawler_LakeFormationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_LakeFormationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `use_lake_formation_credentials` after provisioning.\n"]
    pub fn use_lake_formation_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_lake_formation_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_LineageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_lineage_settings: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCrawler_LineageConfiguration {
    #[doc = "Set the field `crawler_lineage_settings`.\n"]
    pub fn set_crawler_lineage_settings(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.crawler_lineage_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_LineageConfiguration {
    type O = terrars::BlockAssignable<GlueCrawler_LineageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_LineageConfiguration {}

impl BuildGlueCrawler_LineageConfiguration {
    pub fn build(self) -> GlueCrawler_LineageConfiguration {
        GlueCrawler_LineageConfiguration { crawler_lineage_settings: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_LineageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_LineageConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_LineageConfigurationRef {
        GlueCrawler_LineageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_LineageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_lineage_settings` after provisioning.\n"]
    pub fn crawler_lineage_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_lineage_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_MongodbTarget {
    connection_name: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_all: std::option::Option<terrars::PrimField<bool>>,
}

impl GlueCrawler_MongodbTarget {
    #[doc = "Set the field `scan_all`.\n"]
    pub fn set_scan_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.scan_all = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_MongodbTarget {
    type O = terrars::BlockAssignable<GlueCrawler_MongodbTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_MongodbTarget {
    #[doc = ""]
    pub connection_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildGlueCrawler_MongodbTarget {
    pub fn build(self) -> GlueCrawler_MongodbTarget {
        GlueCrawler_MongodbTarget {
            connection_name: self.connection_name,
            path: self.path,
            scan_all: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_MongodbTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_MongodbTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_MongodbTargetRef {
        GlueCrawler_MongodbTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_MongodbTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_all` after provisioning.\n"]
    pub fn scan_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_all", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_RecrawlPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recrawl_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCrawler_RecrawlPolicy {
    #[doc = "Set the field `recrawl_behavior`.\n"]
    pub fn set_recrawl_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recrawl_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_RecrawlPolicy {
    type O = terrars::BlockAssignable<GlueCrawler_RecrawlPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_RecrawlPolicy {}

impl BuildGlueCrawler_RecrawlPolicy {
    pub fn build(self) -> GlueCrawler_RecrawlPolicy {
        GlueCrawler_RecrawlPolicy { recrawl_behavior: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_RecrawlPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_RecrawlPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_RecrawlPolicyRef {
        GlueCrawler_RecrawlPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_RecrawlPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recrawl_behavior` after provisioning.\n"]
    pub fn recrawl_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recrawl_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_S3Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dlq_event_queue_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_queue_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_size: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueCrawler_S3Target {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dlq_event_queue_arn`.\n"]
    pub fn set_dlq_event_queue_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dlq_event_queue_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_queue_arn`.\n"]
    pub fn set_event_queue_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_queue_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_size`.\n"]
    pub fn set_sample_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_S3Target {
    type O = terrars::BlockAssignable<GlueCrawler_S3Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_S3Target {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildGlueCrawler_S3Target {
    pub fn build(self) -> GlueCrawler_S3Target {
        GlueCrawler_S3Target {
            connection_name: core::default::Default::default(),
            dlq_event_queue_arn: core::default::Default::default(),
            event_queue_arn: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            path: self.path,
            sample_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_S3TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_S3TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_S3TargetRef {
        GlueCrawler_S3TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_S3TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dlq_event_queue_arn` after provisioning.\n"]
    pub fn dlq_event_queue_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dlq_event_queue_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `event_queue_arn` after provisioning.\n"]
    pub fn event_queue_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_queue_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_size` after provisioning.\n"]
    pub fn sample_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_size", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCrawler_SchemaChangePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCrawler_SchemaChangePolicy {
    #[doc = "Set the field `delete_behavior`.\n"]
    pub fn set_delete_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_behavior`.\n"]
    pub fn set_update_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCrawler_SchemaChangePolicy {
    type O = terrars::BlockAssignable<GlueCrawler_SchemaChangePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCrawler_SchemaChangePolicy {}

impl BuildGlueCrawler_SchemaChangePolicy {
    pub fn build(self) -> GlueCrawler_SchemaChangePolicy {
        GlueCrawler_SchemaChangePolicy {
            delete_behavior: core::default::Default::default(),
            update_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCrawler_SchemaChangePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCrawler_SchemaChangePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCrawler_SchemaChangePolicyRef {
        GlueCrawler_SchemaChangePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCrawler_SchemaChangePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_behavior` after provisioning.\n"]
    pub fn delete_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `update_behavior` after provisioning.\n"]
    pub fn update_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_behavior", self.base))
    }
}
