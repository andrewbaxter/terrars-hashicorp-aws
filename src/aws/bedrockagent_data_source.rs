#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentDataSourceData_ {
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
    data_deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    knowledge_base_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_configuration: std::option::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_ServerSideEncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentDataSource_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_ingestion_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_VectorIngestionConfiguration>>,
    dynamic: BedrockagentDataSourceDynamic,
}

struct BedrockagentDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentDataSourceData_>,
}

#[derive(Clone)]
pub struct BedrockagentDataSource(std::rc::Rc<BedrockagentDataSource_>);

impl BedrockagentDataSource {
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

    #[doc = "Set the field `data_deletion_policy`.\n"]
    pub fn set_data_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_configuration`.\n"]
    pub fn set_data_source_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption_configuration`.\n"]
    pub fn set_server_side_encryption_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentDataSource_ServerSideEncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentDataSource_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vector_ingestion_configuration`.\n"]
    pub fn set_vector_ingestion_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_ingestion_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vector_ingestion_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `data_deletion_policy` after provisioning.\n"]
    pub fn data_deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(&self) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_ServerSideEncryptionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentDataSource_TimeoutsRef {
        BedrockagentDataSource_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_ingestion_configuration` after provisioning.\n"]
    pub fn vector_ingestion_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_VectorIngestionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.vector_ingestion_configuration", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentDataSource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentDataSource { }

impl terrars::ToListMappable for BedrockagentDataSource {
    type O = terrars::ListRef<BedrockagentDataSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentDataSource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_data_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentDataSource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub knowledge_base_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentDataSource {
        let out = BedrockagentDataSource(std::rc::Rc::new(BedrockagentDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentDataSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                knowledge_base_id: self.knowledge_base_id,
                name: self.name,
                region: core::default::Default::default(),
                data_source_configuration: core::default::Default::default(),
                server_side_encryption_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_ingestion_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentDataSourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_deletion_policy` after provisioning.\n"]
    pub fn data_deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(&self) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_ServerSideEncryptionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentDataSource_TimeoutsRef {
        BedrockagentDataSource_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_ingestion_configuration` after provisioning.\n"]
    pub fn vector_ingestion_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_VectorIngestionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.vector_ingestion_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDynamic {
    data_source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration>>,
    server_side_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_ServerSideEncryptionConfiguration>>,
    vector_ingestion_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_VectorIngestionConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_filters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_filters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    object_type: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
    #[doc = "Set the field `exclusion_filters`.\n"]
    pub fn set_exclusion_filters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusion_filters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_filters`.\n"]
    pub fn set_inclusion_filters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inclusion_filters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
    #[doc = ""]
    pub object_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters {
            exclusion_filters: core::default::Default::default(),
            inclusion_filters: core::default::Default::default(),
            object_type: self.object_type,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_FiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_FiltersRef {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_filters` after provisioning.\n"]
    pub fn exclusion_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_filters` after provisioning.\n"]
    pub fn inclusion_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type` after provisioning.\n"]
    pub fn object_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterDynamic {
    filters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_Filters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter {
            filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterRef {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter_FiltersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationDynamic {
    pattern_object_filter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern_object_filter: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
    #[doc = "Set the field `pattern_object_filter`.\n"]
    pub fn set_pattern_object_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pattern_object_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pattern_object_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration {
            type_: self.type_,
            pattern_object_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_object_filter` after provisioning.\n"]
    pub fn pattern_object_filter(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration_PatternObjectFilterRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pattern_object_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationDynamic {
    filter_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {
    #[doc = "Set the field `filter_configuration`.\n"]
    pub fn set_filter_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration {
            filter_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_configuration` after provisioning.\n"]
    pub fn filter_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration_FilterConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration {
    auth_type: terrars::PrimField<std::string::String>,
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    host_type: terrars::PrimField<std::string::String>,
    host_url: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration { }

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host_url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration {
            auth_type: self.auth_type,
            credentials_secret_arn: self.credentials_secret_arn,
            host_type: self.host_type,
            host_url: self.host_url,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `host_type` after provisioning.\n"]
    pub fn host_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_url` after provisioning.\n"]
    pub fn host_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationDynamic {
    crawler_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration>,
    >,
    source_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration>,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crawler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crawler_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
    deletion_protection_status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
    #[doc = "Set the field `deletion_protection_threshold`.\n"]
    pub fn set_deletion_protection_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.deletion_protection_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
    #[doc = ""]
    pub deletion_protection_status: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration {
            deletion_protection_status: self.deletion_protection_status,
            deletion_protection_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deletion_protection_status` after provisioning.\n"]
    pub fn deletion_protection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_status", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_threshold` after provisioning.\n"]
    pub fn deletion_protection_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {
    audio_extraction_status: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {

}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {
    #[doc = ""]
    pub audio_extraction_status: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration {
            audio_extraction_status: self.audio_extraction_status,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_extraction_status` after provisioning.\n"]
    pub fn audio_extraction_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_extraction_status", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {
    image_extraction_status: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {

}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {
    #[doc = ""]
    pub image_extraction_status: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration {
            image_extraction_status: self.image_extraction_status,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_extraction_status` after provisioning.\n"]
    pub fn image_extraction_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_extraction_status", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {
    video_extraction_status: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {

}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {
    #[doc = ""]
    pub video_extraction_status: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration {
            video_extraction_status: self.video_extraction_status,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `video_extraction_status` after provisioning.\n"]
    pub fn video_extraction_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.video_extraction_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationDynamic {
    audio_extraction_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration,
        >,
    >,
    image_extraction_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration,
        >,
    >,
    video_extraction_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_extraction_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_extraction_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_extraction_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {
    #[doc = "Set the field `audio_extraction_configuration`.\n"]
    pub fn set_audio_extraction_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_extraction_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_extraction_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_extraction_configuration`.\n"]
    pub fn set_image_extraction_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_extraction_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_extraction_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_extraction_configuration`.\n"]
    pub fn set_video_extraction_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_extraction_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_extraction_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration {
            audio_extraction_configuration: core::default::Default::default(),
            image_extraction_configuration: core::default::Default::default(),
            video_extraction_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_extraction_configuration` after provisioning.\n"]
    pub fn audio_extraction_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_AudioExtractionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_extraction_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `image_extraction_configuration` after provisioning.\n"]
    pub fn image_extraction_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_ImageExtractionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_extraction_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `video_extraction_configuration` after provisioning.\n"]
    pub fn video_extraction_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration_VideoExtractionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_extraction_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily {}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily { }

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_DailyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_DailyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_DailyRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_DailyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_DailyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {

}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonthRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthDynamic {
    last_day_of_month: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_day_of_month: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {
    #[doc = "Set the field `day_number`.\n"]
    pub fn set_day_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_day_of_month`.\n"]
    pub fn set_last_day_of_month(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonth,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_day_of_month = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_day_of_month = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth {
            day_number: core::default::Default::default(),
            last_day_of_month: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_number` after provisioning.\n"]
    pub fn day_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_number", self.base))
    }

    #[doc = "Get a reference to the value of field `last_day_of_month` after provisioning.\n"]
    pub fn last_day_of_month(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth_LastDayOfMonthRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_day_of_month", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyDynamic {
    day_of_month: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_month: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonth,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.day_of_month = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.day_of_month = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly {
            day_of_month: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly_DayOfMonthRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly {
    day_of_week: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly { }

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly {
    #[doc = ""]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly {
            day_of_week: self.day_of_week,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_WeeklyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_WeeklyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_WeeklyRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_WeeklyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_WeeklyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleDynamic {
    daily: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily,
        >,
    >,
    monthly: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly,
        >,
    >,
    weekly: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {
    #[doc = "Set the field `daily`.\n"]
    pub fn set_daily(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Daily,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.daily = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.daily = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monthly`.\n"]
    pub fn set_monthly(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Monthly,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monthly = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly`.\n"]
    pub fn set_weekly(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_Weekly,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule {
            daily: core::default::Default::default(),
            monthly: core::default::Default::default(),
            weekly: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily` after provisioning.\n"]
    pub fn daily(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_DailyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly` after provisioning.\n"]
    pub fn monthly(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_MonthlyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly` after provisioning.\n"]
    pub fn weekly(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule_WeeklyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationDynamic {
    deletion_protection_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration,
        >,
    >,
    media_extraction_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration,
        >,
    >,
    sync_schedule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_extraction_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_schedule: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule>,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {
    #[doc = "Set the field `connector_parameters`.\n"]
    pub fn set_connector_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection_configuration`.\n"]
    pub fn set_deletion_protection_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deletion_protection_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deletion_protection_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `media_extraction_configuration`.\n"]
    pub fn set_media_extraction_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.media_extraction_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.media_extraction_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sync_schedule`.\n"]
    pub fn set_sync_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncSchedule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sync_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sync_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration {
            connector_parameters: core::default::Default::default(),
            deletion_protection_configuration: core::default::Default::default(),
            media_extraction_configuration: core::default::Default::default(),
            sync_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_parameters` after provisioning.\n"]
    pub fn connector_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_configuration` after provisioning.\n"]
    pub fn deletion_protection_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_DeletionProtectionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deletion_protection_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `media_extraction_configuration` after provisioning.\n"]
    pub fn media_extraction_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_MediaExtractionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_extraction_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_schedule` after provisioning.\n"]
    pub fn sync_schedule(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration_SyncScheduleRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sync_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_S3Configuration {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentDataSource_DataSourceConfiguration_S3Configuration {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_owner_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_prefixes`.\n"]
    pub fn set_inclusion_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inclusion_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_S3Configuration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_S3Configuration {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_S3Configuration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_S3Configuration {
        BedrockagentDataSource_DataSourceConfiguration_S3Configuration {
            bucket_arn: self.bucket_arn,
            bucket_owner_account_id: core::default::Default::default(),
            inclusion_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_S3ConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_prefixes` after provisioning.\n"]
    pub fn inclusion_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inclusion_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration {
    auth_type: terrars::PrimField<std::string::String>,
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    host_url: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration { }

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host_url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration {
            auth_type: self.auth_type,
            credentials_secret_arn: self.credentials_secret_arn,
            host_url: self.host_url,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `host_url` after provisioning.\n"]
    pub fn host_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationDynamic {
    crawler_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration>,
    >,
    source_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration>,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crawler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crawler_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
    auth_type: terrars::PrimField<std::string::String>,
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    domain: terrars::PrimField<std::string::String>,
    host_type: terrars::PrimField<std::string::String>,
    site_urls: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub site_urls: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration {
            auth_type: self.auth_type,
            credentials_secret_arn: self.credentials_secret_arn,
            domain: self.domain,
            host_type: self.host_type,
            site_urls: self.site_urls,
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `host_type` after provisioning.\n"]
    pub fn host_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_type", self.base))
    }

    #[doc = "Get a reference to the value of field `site_urls` after provisioning.\n"]
    pub fn site_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.site_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationDynamic {
    crawler_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration>,
    >,
    source_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration>,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crawler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration_CrawlerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crawler_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {
    #[doc = "Set the field `max_pages`.\n"]
    pub fn set_max_pages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit`.\n"]
    pub fn set_rate_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rate_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits {
            max_pages: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimitsRef {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_pages` after provisioning.\n"]
    pub fn max_pages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limit", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationDynamic {
    crawler_limits: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_filters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_filters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_limits: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits>,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {
    #[doc = "Set the field `exclusion_filters`.\n"]
    pub fn set_exclusion_filters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusion_filters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_filters`.\n"]
    pub fn set_inclusion_filters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inclusion_filters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_agent`.\n"]
    pub fn set_user_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crawler_limits`.\n"]
    pub fn set_crawler_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimits,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crawler_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_limits = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration {
            exclusion_filters: core::default::Default::default(),
            inclusion_filters: core::default::Default::default(),
            scope: core::default::Default::default(),
            user_agent: core::default::Default::default(),
            crawler_limits: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_filters` after provisioning.\n"]
    pub fn exclusion_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_filters` after provisioning.\n"]
    pub fn inclusion_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `user_agent` after provisioning.\n"]
    pub fn user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `crawler_limits` after provisioning.\n"]
    pub fn crawler_limits(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration_CrawlerLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crawler_limits", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls {
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrlsRef {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationDynamic {
    seed_urls: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seed_urls: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls,
        >,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {
    #[doc = "Set the field `seed_urls`.\n"]
    pub fn set_seed_urls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrls,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.seed_urls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.seed_urls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration {
            seed_urls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed_urls` after provisioning.\n"]
    pub fn seed_urls(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration_SeedUrlsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.seed_urls", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationDynamic {
    url_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration>,
    >,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {
    #[doc = "Set the field `url_configuration`.\n"]
    pub fn set_url_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration {
            url_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url_configuration` after provisioning.\n"]
    pub fn url_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration_UrlConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfiguration_WebConfigurationDynamic {
    crawler_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration>,
    >,
    source_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration>>,
    dynamic: BedrockagentDataSource_DataSourceConfiguration_WebConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfiguration {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crawler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration_WebConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration {}

impl BuildBedrockagentDataSource_DataSourceConfiguration_WebConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration_WebConfiguration {
        BedrockagentDataSource_DataSourceConfiguration_WebConfiguration {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfiguration_WebConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfiguration_WebConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfiguration_WebConfigurationRef {
        BedrockagentDataSource_DataSourceConfiguration_WebConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfiguration_WebConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_CrawlerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crawler_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration_SourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_DataSourceConfigurationDynamic {
    confluence_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration>>,
    managed_knowledge_base_connector_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration>,
    >,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_S3Configuration>>,
    salesforce_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration>>,
    share_point_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration>>,
    web_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_DataSourceConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confluence_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_knowledge_base_connector_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std::option::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_point_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration>>,
    dynamic: BedrockagentDataSource_DataSourceConfigurationDynamic,
}

impl BedrockagentDataSource_DataSourceConfiguration {
    #[doc = "Set the field `confluence_configuration`.\n"]
    pub fn set_confluence_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confluence_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confluence_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_knowledge_base_connector_configuration`.\n"]
    pub fn set_managed_knowledge_base_connector_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_knowledge_base_connector_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_knowledge_base_connector_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_S3Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce_configuration`.\n"]
    pub fn set_salesforce_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `share_point_configuration`.\n"]
    pub fn set_share_point_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_SharePointConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.share_point_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.share_point_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_configuration`.\n"]
    pub fn set_web_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration_WebConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_DataSourceConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_DataSourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_DataSourceConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_DataSourceConfiguration {
    pub fn build(self) -> BedrockagentDataSource_DataSourceConfiguration {
        BedrockagentDataSource_DataSourceConfiguration {
            type_: self.type_,
            confluence_configuration: core::default::Default::default(),
            managed_knowledge_base_connector_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            salesforce_configuration: core::default::Default::default(),
            share_point_configuration: core::default::Default::default(),
            web_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_DataSourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_DataSourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_DataSourceConfigurationRef {
        BedrockagentDataSource_DataSourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_DataSourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `confluence_configuration` after provisioning.\n"]
    pub fn confluence_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_ConfluenceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confluence_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_knowledge_base_connector_configuration` after provisioning.\n"]
    pub fn managed_knowledge_base_connector_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentDataSource_DataSourceConfiguration_ManagedKnowledgeBaseConnectorConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.managed_knowledge_base_connector_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce_configuration` after provisioning.\n"]
    pub fn salesforce_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_SalesforceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `share_point_configuration` after provisioning.\n"]
    pub fn share_point_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_SharePointConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_point_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `web_configuration` after provisioning.\n"]
    pub fn web_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_DataSourceConfiguration_WebConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_ServerSideEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentDataSource_ServerSideEncryptionConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_ServerSideEncryptionConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_ServerSideEncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_ServerSideEncryptionConfiguration {}

impl BuildBedrockagentDataSource_ServerSideEncryptionConfiguration {
    pub fn build(self) -> BedrockagentDataSource_ServerSideEncryptionConfiguration {
        BedrockagentDataSource_ServerSideEncryptionConfiguration { kms_key_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_ServerSideEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_ServerSideEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_ServerSideEncryptionConfigurationRef {
        BedrockagentDataSource_ServerSideEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_ServerSideEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentDataSource_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentDataSource_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_Timeouts {}

impl BuildBedrockagentDataSource_Timeouts {
    pub fn build(self) -> BedrockagentDataSource_Timeouts {
        BedrockagentDataSource_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentDataSource_TimeoutsRef {
        BedrockagentDataSource_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration {
    max_tokens: terrars::PrimField<f64>,
    overlap_percentage: terrars::PrimField<f64>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration { }

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration {
    #[doc = ""]
    pub max_tokens: terrars::PrimField<f64>,
    #[doc = ""]
    pub overlap_percentage: terrars::PrimField<f64>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration {
            max_tokens: self.max_tokens,
            overlap_percentage: self.overlap_percentage,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `overlap_percentage` after provisioning.\n"]
    pub fn overlap_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overlap_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {
    max_tokens: terrars::PrimField<f64>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {

}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {
    #[doc = ""]
    pub max_tokens: terrars::PrimField<f64>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration {
            max_tokens: self.max_tokens,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationDynamic {
    level_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
    overlap_tokens: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
    #[doc = "Set the field `level_configuration`.\n"]
    pub fn set_level_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.level_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.level_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
    #[doc = ""]
    pub overlap_tokens: terrars::PrimField<f64>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration {
            overlap_tokens: self.overlap_tokens,
            level_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `overlap_tokens` after provisioning.\n"]
    pub fn overlap_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overlap_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `level_configuration` after provisioning.\n"]
    pub fn level_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration_LevelConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration {
    breakpoint_percentile_threshold: terrars::PrimField<f64>,
    buffer_size: terrars::PrimField<f64>,
    max_token: terrars::PrimField<f64>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration { }

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration {
    #[doc = ""]
    pub breakpoint_percentile_threshold: terrars::PrimField<f64>,
    #[doc = ""]
    pub buffer_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_token: terrars::PrimField<f64>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration {
            breakpoint_percentile_threshold: self.breakpoint_percentile_threshold,
            buffer_size: self.buffer_size,
            max_token: self.max_token,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `breakpoint_percentile_threshold` after provisioning.\n"]
    pub fn breakpoint_percentile_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.breakpoint_percentile_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
    }

    #[doc = "Get a reference to the value of field `max_token` after provisioning.\n"]
    pub fn max_token(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_token", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationDynamic {
    fixed_size_chunking_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration,
        >,
    >,
    hierarchical_chunking_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration,
        >,
    >,
    semantic_chunking_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
    chunking_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_size_chunking_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchical_chunking_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    semantic_chunking_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration>,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
    #[doc = "Set the field `fixed_size_chunking_configuration`.\n"]
    pub fn set_fixed_size_chunking_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_size_chunking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_size_chunking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hierarchical_chunking_configuration`.\n"]
    pub fn set_hierarchical_chunking_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hierarchical_chunking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hierarchical_chunking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `semantic_chunking_configuration`.\n"]
    pub fn set_semantic_chunking_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.semantic_chunking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.semantic_chunking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
    #[doc = ""]
    pub chunking_strategy: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
    pub fn build(self) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration {
            chunking_strategy: self.chunking_strategy,
            fixed_size_chunking_configuration: core::default::Default::default(),
            hierarchical_chunking_configuration: core::default::Default::default(),
            semantic_chunking_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunking_strategy` after provisioning.\n"]
    pub fn chunking_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunking_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_size_chunking_configuration` after provisioning.\n"]
    pub fn fixed_size_chunking_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_FixedSizeChunkingConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_size_chunking_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `hierarchical_chunking_configuration` after provisioning.\n"]
    pub fn hierarchical_chunking_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_HierarchicalChunkingConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchical_chunking_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `semantic_chunking_configuration` after provisioning.\n"]
    pub fn semantic_chunking_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration_SemanticChunkingConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.semantic_chunking_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {
    method: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {

}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {
    #[doc = ""]
    pub method: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration {
            method: self.method,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationDynamic {
    enrichment_strategy_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
    model_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enrichment_strategy_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
    #[doc = "Set the field `enrichment_strategy_configuration`.\n"]
    pub fn set_enrichment_strategy_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enrichment_strategy_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enrichment_strategy_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
    #[doc = ""]
    pub model_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration {
            model_arn: self.model_arn,
            enrichment_strategy_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enrichment_strategy_configuration` after provisioning.\n"]
    pub fn enrichment_strategy_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration_EnrichmentStrategyConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enrichment_strategy_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationDynamic {
    bedrock_foundation_model_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_foundation_model_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
    #[doc = "Set the field `bedrock_foundation_model_configuration`.\n"]
    pub fn set_bedrock_foundation_model_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_foundation_model_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_foundation_model_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
    type O =
        terrars::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
    pub fn build(self) -> BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration {
            type_: self.type_,
            bedrock_foundation_model_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `bedrock_foundation_model_configuration` after provisioning.\n"]
    pub fn bedrock_foundation_model_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration_BedrockFoundationModelConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_foundation_model_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {
    uri: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {

}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location {
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3LocationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageDynamic {
    s3_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_location: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {
    #[doc = "Set the field `s3_location`.\n"]
    pub fn set_s3_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3Location,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage {
            s3_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageRef {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_location` after provisioning.\n"]
    pub fn s3_location(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage_S3LocationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {
    lambda_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {

}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration {
            lambda_arn: self.lambda_arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionDynamic {
    transformation_lambda_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_lambda_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {
    #[doc = "Set the field `transformation_lambda_configuration`.\n"]
    pub fn set_transformation_lambda_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation_lambda_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_lambda_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction {
            transformation_lambda_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionRef {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `transformation_lambda_configuration` after provisioning.\n"]
    pub fn transformation_lambda_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction_TransformationLambdaConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_lambda_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationDynamic {
    transformation_function: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
    step_to_apply: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_function: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
    #[doc = "Set the field `transformation_function`.\n"]
    pub fn set_transformation_function(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunction,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_function = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
    #[doc = ""]
    pub step_to_apply: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation {
            step_to_apply: self.step_to_apply,
            transformation_function: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `step_to_apply` after provisioning.\n"]
    pub fn step_to_apply(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_to_apply", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation_function` after provisioning.\n"]
    pub fn transformation_function(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation_TransformationFunctionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_function", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationDynamic {
    intermediate_storage: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage,
        >,
    >,
    transformation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intermediate_storage: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation>,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {
    #[doc = "Set the field `intermediate_storage`.\n"]
    pub fn set_intermediate_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.intermediate_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.intermediate_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transformation`.\n"]
    pub fn set_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_Transformation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {
    pub fn build(self) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration {
            intermediate_storage: core::default::Default::default(),
            transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intermediate_storage` after provisioning.\n"]
    pub fn intermediate_storage(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_IntermediateStorageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intermediate_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation` after provisioning.\n"]
    pub fn transformation(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration_TransformationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parsing_modality: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {
    #[doc = "Set the field `parsing_modality`.\n"]
    pub fn set_parsing_modality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parsing_modality = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration {
            parsing_modality: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parsing_modality` after provisioning.\n"]
    pub fn parsing_modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parsing_modality", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {
    parsing_prompt_string: terrars::PrimField<std::string::String>,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {

}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {
    #[doc = ""]
    pub parsing_prompt_string: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt {
            parsing_prompt_string: self.parsing_prompt_string,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPromptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPromptRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parsing_prompt_string` after provisioning.\n"]
    pub fn parsing_prompt_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parsing_prompt_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationDynamic {
    parsing_prompt: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
    model_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parsing_modality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parsing_prompt: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
    #[doc = "Set the field `parsing_modality`.\n"]
    pub fn set_parsing_modality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parsing_modality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parsing_prompt`.\n"]
    pub fn set_parsing_prompt(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPrompt,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parsing_prompt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parsing_prompt = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
    #[doc = ""]
    pub model_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration {
            model_arn: self.model_arn,
            parsing_modality: core::default::Default::default(),
            parsing_prompt: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `parsing_modality` after provisioning.\n"]
    pub fn parsing_modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parsing_modality", self.base))
    }

    #[doc = "Get a reference to the value of field `parsing_prompt` after provisioning.\n"]
    pub fn parsing_prompt(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration_ParsingPromptRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parsing_prompt", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationDynamic {
    bedrock_data_automation_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration,
        >,
    >,
    bedrock_foundation_model_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
    parsing_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_data_automation_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_foundation_model_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration,
        >,
    >,
    dynamic: BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
    #[doc = "Set the field `bedrock_data_automation_configuration`.\n"]
    pub fn set_bedrock_data_automation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_data_automation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_data_automation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bedrock_foundation_model_configuration`.\n"]
    pub fn set_bedrock_foundation_model_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_foundation_model_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_foundation_model_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
    #[doc = ""]
    pub parsing_strategy: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
    pub fn build(self) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration {
            parsing_strategy: self.parsing_strategy,
            bedrock_data_automation_configuration: core::default::Default::default(),
            bedrock_foundation_model_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parsing_strategy` after provisioning.\n"]
    pub fn parsing_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parsing_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `bedrock_data_automation_configuration` after provisioning.\n"]
    pub fn bedrock_data_automation_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockDataAutomationConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_data_automation_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `bedrock_foundation_model_configuration` after provisioning.\n"]
    pub fn bedrock_foundation_model_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration_BedrockFoundationModelConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_foundation_model_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSource_VectorIngestionConfigurationDynamic {
    chunking_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration>>,
    context_enrichment_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration>,
    >,
    custom_transformation_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration>,
    >,
    parsing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentDataSource_VectorIngestionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunking_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_enrichment_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_transformation_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parsing_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration>>,
    dynamic: BedrockagentDataSource_VectorIngestionConfigurationDynamic,
}

impl BedrockagentDataSource_VectorIngestionConfiguration {
    #[doc = "Set the field `chunking_configuration`.\n"]
    pub fn set_chunking_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.chunking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.chunking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `context_enrichment_configuration`.\n"]
    pub fn set_context_enrichment_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.context_enrichment_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.context_enrichment_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_transformation_configuration`.\n"]
    pub fn set_custom_transformation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_transformation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_transformation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parsing_configuration`.\n"]
    pub fn set_parsing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parsing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parsing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentDataSource_VectorIngestionConfiguration {
    type O = terrars::BlockAssignable<BedrockagentDataSource_VectorIngestionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSource_VectorIngestionConfiguration {}

impl BuildBedrockagentDataSource_VectorIngestionConfiguration {
    pub fn build(self) -> BedrockagentDataSource_VectorIngestionConfiguration {
        BedrockagentDataSource_VectorIngestionConfiguration {
            chunking_configuration: core::default::Default::default(),
            context_enrichment_configuration: core::default::Default::default(),
            custom_transformation_configuration: core::default::Default::default(),
            parsing_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentDataSource_VectorIngestionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentDataSource_VectorIngestionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentDataSource_VectorIngestionConfigurationRef {
        BedrockagentDataSource_VectorIngestionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSource_VectorIngestionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunking_configuration` after provisioning.\n"]
    pub fn chunking_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_VectorIngestionConfiguration_ChunkingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chunking_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `context_enrichment_configuration` after provisioning.\n"]
    pub fn context_enrichment_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_VectorIngestionConfiguration_ContextEnrichmentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.context_enrichment_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_transformation_configuration` after provisioning.\n"]
    pub fn custom_transformation_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_VectorIngestionConfiguration_CustomTransformationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_transformation_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `parsing_configuration` after provisioning.\n"]
    pub fn parsing_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentDataSource_VectorIngestionConfiguration_ParsingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parsing_configuration", self.base))
    }
}
