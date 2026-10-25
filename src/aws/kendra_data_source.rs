#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KendraDataSourceData_ {
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
    index_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<KendraDataSource_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_document_enrichment_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KendraDataSource_Timeouts>,
    dynamic: KendraDataSourceDynamic,
}

struct KendraDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KendraDataSourceData_>,
}

#[derive(Clone)]
pub struct KendraDataSource(std::rc::Rc<KendraDataSource_>);

impl KendraDataSource {
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

    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSource_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_document_enrichment_configuration`.\n"]
    pub fn set_custom_document_enrichment_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_document_enrichment_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_document_enrichment_configuration =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KendraDataSource_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<KendraDataSource_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_document_enrichment_configuration` after provisioning.\n"]
    pub fn custom_document_enrichment_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_CustomDocumentEnrichmentConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.custom_document_enrichment_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraDataSource_TimeoutsRef {
        KendraDataSource_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for KendraDataSource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KendraDataSource { }

impl terrars::ToListMappable for KendraDataSource {
    type O = terrars::ListRef<KendraDataSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KendraDataSource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kendra_data_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKendraDataSource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub index_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource {
    pub fn build(self, stack: &mut terrars::Stack) -> KendraDataSource {
        let out = KendraDataSource(std::rc::Rc::new(KendraDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KendraDataSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                index_id: self.index_id,
                language_code: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                schedule: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                configuration: core::default::Default::default(),
                custom_document_enrichment_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KendraDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KendraDataSourceRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<KendraDataSource_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_document_enrichment_configuration` after provisioning.\n"]
    pub fn custom_document_enrichment_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_CustomDocumentEnrichmentConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.custom_document_enrichment_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraDataSource_TimeoutsRef {
        KendraDataSource_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<KendraDataSource_Configuration>>,
    custom_document_enrichment_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_CustomDocumentEnrichmentConfiguration>>,
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {
    #[doc = "Set the field `key_path`.\n"]
    pub fn set_key_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {}

impl BuildKendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {
        KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration {
            key_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_S3Configuration_AccessControlListConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_S3Configuration_AccessControlListConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_S3Configuration_AccessControlListConfigurationRef {
        KendraDataSource_Configuration_S3Configuration_AccessControlListConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_S3Configuration_AccessControlListConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_path` after provisioning.\n"]
    pub fn key_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_path", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {
    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {}

impl BuildKendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {
        KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration {
            s3_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfigurationRef {
        KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_Configuration_S3ConfigurationDynamic {
    access_control_list_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration>>,
    documents_metadata_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration>>,
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_S3Configuration {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_patterns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_patterns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_list_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_metadata_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration>>,
    dynamic: KendraDataSource_Configuration_S3ConfigurationDynamic,
}

impl KendraDataSource_Configuration_S3Configuration {
    #[doc = "Set the field `exclusion_patterns`.\n"]
    pub fn set_exclusion_patterns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusion_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_patterns`.\n"]
    pub fn set_inclusion_patterns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inclusion_patterns = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `access_control_list_configuration`.\n"]
    pub fn set_access_control_list_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_Configuration_S3Configuration_AccessControlListConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_list_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_list_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `documents_metadata_configuration`.\n"]
    pub fn set_documents_metadata_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.documents_metadata_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.documents_metadata_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_S3Configuration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_S3Configuration {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource_Configuration_S3Configuration {
    pub fn build(self) -> KendraDataSource_Configuration_S3Configuration {
        KendraDataSource_Configuration_S3Configuration {
            bucket_name: self.bucket_name,
            exclusion_patterns: core::default::Default::default(),
            inclusion_patterns: core::default::Default::default(),
            inclusion_prefixes: core::default::Default::default(),
            access_control_list_configuration: core::default::Default::default(),
            documents_metadata_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_S3ConfigurationRef {
        KendraDataSource_Configuration_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_patterns` after provisioning.\n"]
    pub fn exclusion_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclusion_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_patterns` after provisioning.\n"]
    pub fn inclusion_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inclusion_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_prefixes` after provisioning.\n"]
    pub fn inclusion_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inclusion_prefixes", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_list_configuration` after provisioning.\n"]
    pub fn access_control_list_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_S3Configuration_AccessControlListConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_list_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `documents_metadata_configuration` after provisioning.\n"]
    pub fn documents_metadata_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_S3Configuration_DocumentsMetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documents_metadata_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_TemplateConfiguration {
    template: terrars::PrimField<std::string::String>,
}

impl KendraDataSource_Configuration_TemplateConfiguration { }

impl terrars::ToListMappable for KendraDataSource_Configuration_TemplateConfiguration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_TemplateConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_TemplateConfiguration {
    #[doc = ""]
    pub template: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource_Configuration_TemplateConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_TemplateConfiguration {
        KendraDataSource_Configuration_TemplateConfiguration { template: self.template }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_TemplateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_TemplateConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_TemplateConfigurationRef {
        KendraDataSource_Configuration_TemplateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_TemplateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication {
    credentials: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication { }

impl terrars
::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication {
    #[doc = ""]
    pub credentials: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication {
    pub fn build(
        self,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication {
        KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication {
            credentials: self.credentials,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthenticationRef {
        KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationDynamic {
    basic_authentication: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_authentication: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication>,
    >,
    dynamic: KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationDynamic,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {
    #[doc = "Set the field `basic_authentication`.\n"]
    pub fn set_basic_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration_BasicAuthentication,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {
    type O =
        terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {
        KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration {
            basic_authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationRef {
        KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
        KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration {
            credentials: core::default::Default::default(),
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfigurationRef {
        KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
    seed_urls: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
    #[doc = "Set the field `web_crawler_mode`.\n"]
    pub fn set_web_crawler_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_crawler_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
    type O =
        terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
    #[doc = ""]
    pub seed_urls: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
        KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration {
            seed_urls: self.seed_urls,
            web_crawler_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfigurationRef {
        KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed_urls` after provisioning.\n"]
    pub fn seed_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.seed_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `web_crawler_mode` after provisioning.\n"]
    pub fn web_crawler_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_crawler_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration {
    site_maps: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration { }

impl terrars::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration {
    type O =
        terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration {
    #[doc = ""]
    pub site_maps: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration {
        KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration {
            site_maps: self.site_maps,
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfigurationRef {
        KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_maps` after provisioning.\n"]
    pub fn site_maps(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.site_maps", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsDynamic {
    seed_url_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration>>,
    site_maps_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_Urls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seed_url_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    site_maps_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration>>,
    dynamic: KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsDynamic,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_Urls {
    #[doc = "Set the field `seed_url_configuration`.\n"]
    pub fn set_seed_url_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.seed_url_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.seed_url_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `site_maps_configuration`.\n"]
    pub fn set_site_maps_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.site_maps_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.site_maps_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration_Urls {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration_Urls {}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration_Urls {
    pub fn build(self) -> KendraDataSource_Configuration_WebCrawlerConfiguration_Urls {
        KendraDataSource_Configuration_WebCrawlerConfiguration_Urls {
            seed_url_configuration: core::default::Default::default(),
            site_maps_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsRef {
        KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed_url_configuration` after provisioning.\n"]
    pub fn seed_url_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SeedUrlConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.seed_url_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `site_maps_configuration` after provisioning.\n"]
    pub fn site_maps_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls_SiteMapsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.site_maps_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_Configuration_WebCrawlerConfigurationDynamic {
    authentication_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration>,
    >,
    proxy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration>>,
    urls: std::option::Option<terrars::DynamicBlock<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls>>,
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration_WebCrawlerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawl_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_content_size_per_page_in_mega_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_links_per_page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_urls_per_minute_crawl_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_exclusion_patterns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_inclusion_patterns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    urls: std::option::Option<std::vec::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls>>,
    dynamic: KendraDataSource_Configuration_WebCrawlerConfigurationDynamic,
}

impl KendraDataSource_Configuration_WebCrawlerConfiguration {
    #[doc = "Set the field `crawl_depth`.\n"]
    pub fn set_crawl_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.crawl_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_content_size_per_page_in_mega_bytes`.\n"]
    pub fn set_max_content_size_per_page_in_mega_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_content_size_per_page_in_mega_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_links_per_page`.\n"]
    pub fn set_max_links_per_page(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_links_per_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_urls_per_minute_crawl_rate`.\n"]
    pub fn set_max_urls_per_minute_crawl_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_urls_per_minute_crawl_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_exclusion_patterns`.\n"]
    pub fn set_url_exclusion_patterns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.url_exclusion_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_inclusion_patterns`.\n"]
    pub fn set_url_inclusion_patterns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.url_inclusion_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_configuration`.\n"]
    pub fn set_proxy_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proxy_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `urls`.\n"]
    pub fn set_urls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration_Urls>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.urls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.urls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration_WebCrawlerConfiguration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration_WebCrawlerConfiguration {}

impl BuildKendraDataSource_Configuration_WebCrawlerConfiguration {
    pub fn build(self) -> KendraDataSource_Configuration_WebCrawlerConfiguration {
        KendraDataSource_Configuration_WebCrawlerConfiguration {
            crawl_depth: core::default::Default::default(),
            max_content_size_per_page_in_mega_bytes: core::default::Default::default(),
            max_links_per_page: core::default::Default::default(),
            max_urls_per_minute_crawl_rate: core::default::Default::default(),
            url_exclusion_patterns: core::default::Default::default(),
            url_inclusion_patterns: core::default::Default::default(),
            authentication_configuration: core::default::Default::default(),
            proxy_configuration: core::default::Default::default(),
            urls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_Configuration_WebCrawlerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_Configuration_WebCrawlerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_Configuration_WebCrawlerConfigurationRef {
        KendraDataSource_Configuration_WebCrawlerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_Configuration_WebCrawlerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawl_depth` after provisioning.\n"]
    pub fn crawl_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawl_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `max_content_size_per_page_in_mega_bytes` after provisioning.\n"]
    pub fn max_content_size_per_page_in_mega_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_content_size_per_page_in_mega_bytes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_links_per_page` after provisioning.\n"]
    pub fn max_links_per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_links_per_page", self.base))
    }

    #[doc = "Get a reference to the value of field `max_urls_per_minute_crawl_rate` after provisioning.\n"]
    pub fn max_urls_per_minute_crawl_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_urls_per_minute_crawl_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `url_exclusion_patterns` after provisioning.\n"]
    pub fn url_exclusion_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.url_exclusion_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `url_inclusion_patterns` after provisioning.\n"]
    pub fn url_inclusion_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.url_inclusion_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_WebCrawlerConfiguration_AuthenticationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_WebCrawlerConfiguration_ProxyConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `urls` after provisioning.\n"]
    pub fn urls(&self) -> terrars::ListRef<KendraDataSource_Configuration_WebCrawlerConfiguration_UrlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_ConfigurationDynamic {
    s3_configuration: std::option::Option<terrars::DynamicBlock<KendraDataSource_Configuration_S3Configuration>>,
    template_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_Configuration_TemplateConfiguration>>,
    web_crawler_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_Configuration_WebCrawlerConfiguration>>,
}

#[derive(Serialize)]
pub struct KendraDataSource_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std::option::Option<std::vec::Vec<KendraDataSource_Configuration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_configuration: std::option::Option<std::vec::Vec<KendraDataSource_Configuration_TemplateConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_configuration: std::option::Option<std::vec::Vec<KendraDataSource_Configuration_WebCrawlerConfiguration>>,
    dynamic: KendraDataSource_ConfigurationDynamic,
}

impl KendraDataSource_Configuration {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSource_Configuration_S3Configuration>>,
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

    #[doc = "Set the field `template_configuration`.\n"]
    pub fn set_template_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSource_Configuration_TemplateConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_crawler_configuration`.\n"]
    pub fn set_web_crawler_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSource_Configuration_WebCrawlerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_crawler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_crawler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_Configuration {
    type O = terrars::BlockAssignable<KendraDataSource_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Configuration {}

impl BuildKendraDataSource_Configuration {
    pub fn build(self) -> KendraDataSource_Configuration {
        KendraDataSource_Configuration {
            s3_configuration: core::default::Default::default(),
            template_configuration: core::default::Default::default(),
            web_crawler_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraDataSource_ConfigurationRef {
        KendraDataSource_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> terrars::ListRef<KendraDataSource_Configuration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `template_configuration` after provisioning.\n"]
    pub fn template_configuration(&self) -> terrars::ListRef<KendraDataSource_Configuration_TemplateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `web_crawler_configuration` after provisioning.\n"]
    pub fn web_crawler_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_Configuration_WebCrawlerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_crawler_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {
    #[doc = "Set the field `date_value`.\n"]
    pub fn set_date_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.date_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_value`.\n"]
    pub fn set_long_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.string_list_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {
    pub fn build(
        self,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue {
            date_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date_value` after provisioning.\n"]
    pub fn date_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_value", self.base))
    }

    #[doc = "Get a reference to the value of field `long_value` after provisioning.\n"]
    pub fn long_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionDynamic {
    condition_on_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
    condition_document_attribute_key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_on_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue>,
    >,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
    #[doc = "Set the field `condition_on_value`.\n"]
    pub fn set_condition_on_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_on_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_on_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
    type O =
        terrars
        ::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
    #[doc = ""]
    pub condition_document_attribute_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
    pub fn build(self) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition {
            condition_document_attribute_key: self.condition_document_attribute_key,
            operator: self.operator,
            condition_on_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_document_attribute_key` after provisioning.\n"]
    pub fn condition_document_attribute_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_document_attribute_key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_on_value` after provisioning.\n"]
    pub fn condition_on_value(
        &self,
    ) -> terrars
    ::ListRef<
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_on_value", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {
    #[doc = "Set the field `date_value`.\n"]
    pub fn set_date_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.date_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_value`.\n"]
    pub fn set_long_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.long_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.string_list_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {
    pub fn build(
        self,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue {
            date_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValueRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date_value` after provisioning.\n"]
    pub fn date_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_value", self.base))
    }

    #[doc = "Get a reference to the value of field `long_value` after provisioning.\n"]
    pub fn long_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.string_list_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetDynamic {
    target_document_attribute_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_document_attribute_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_document_attribute_value_deletion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_document_attribute_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue,
        >,
    >,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {
    #[doc = "Set the field `target_document_attribute_key`.\n"]
    pub fn set_target_document_attribute_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_document_attribute_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_document_attribute_value_deletion`.\n"]
    pub fn set_target_document_attribute_value_deletion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.target_document_attribute_value_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_document_attribute_value`.\n"]
    pub fn set_target_document_attribute_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_document_attribute_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_document_attribute_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {
    type O =
        terrars::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {
    pub fn build(self) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target {
            target_document_attribute_key: core::default::Default::default(),
            target_document_attribute_value_deletion: core::default::Default::default(),
            target_document_attribute_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_document_attribute_key` after provisioning.\n"]
    pub fn target_document_attribute_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_document_attribute_key", self.base))
    }

    #[doc = "Get a reference to the value of field `target_document_attribute_value_deletion` after provisioning.\n"]
    pub fn target_document_attribute_value_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_document_attribute_value_deletion", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target_document_attribute_value` after provisioning.\n"]
    pub fn target_document_attribute_value(
        &self,
    ) -> terrars
    ::ListRef<
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target_TargetDocumentAttributeValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_document_attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsDynamic {
    condition: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition>,
    >,
    target: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_content_deletion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target>>,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {
    #[doc = "Set the field `document_content_deletion`.\n"]
    pub fn set_document_content_deletion(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.document_content_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Target,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {
    type O = terrars::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {
    pub fn build(self) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations {
            document_content_deletion: core::default::Default::default(),
            condition: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_content_deletion` after provisioning.\n"]
    pub fn document_content_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_content_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(
        &self,
    ) -> terrars::ListRef<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(
        &self,
    ) -> terrars::ListRef<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionDynamic {
    condition_on_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
    condition_document_attribute_key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_on_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue>,
    >,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
    #[doc = "Set the field `condition_on_value`.\n"]
    pub fn set_condition_on_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_on_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_on_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
    #[doc = ""]
    pub condition_document_attribute_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
    pub fn build(
        self,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition {
            condition_document_attribute_key: self.condition_document_attribute_key,
            operator: self.operator,
            condition_on_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_document_attribute_key` after provisioning.\n"]
    pub fn condition_document_attribute_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_document_attribute_key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_on_value` after provisioning.\n"]
    pub fn condition_on_value(
        &self,
    ) -> terrars
    ::ListRef<
        KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations_Condition_ConditionOnValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_on_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationDynamic {
    invocation_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
    lambda_arn: terrars::PrimField<std::string::String>,
    s3_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
        >,
    >,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
    #[doc = "Set the field `invocation_condition`.\n"]
    pub fn set_invocation_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invocation_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
    type O =
        terrars
        ::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
    pub fn build(self) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration {
            lambda_arn: self.lambda_arn,
            s3_bucket: self.s3_bucket,
            invocation_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_condition` after provisioning.\n"]
    pub fn invocation_condition(
        &self,
    ) -> terrars
    ::ListRef<
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationDynamic {
    invocation_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
    lambda_arn: terrars::PrimField<std::string::String>,
    s3_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
        >,
    >,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
    #[doc = "Set the field `invocation_condition`.\n"]
    pub fn set_invocation_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invocation_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
    type O =
        terrars
        ::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
    pub fn build(self) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration {
            lambda_arn: self.lambda_arn,
            s3_bucket: self.s3_bucket,
            invocation_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationRef {
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_condition` after provisioning.\n"]
    pub fn invocation_condition(
        &self,
    ) -> terrars
    ::ListRef<
        KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration_InvocationConditionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSource_CustomDocumentEnrichmentConfigurationDynamic {
    inline_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations>>,
    post_extraction_hook_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration>,
    >,
    pre_extraction_hook_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_configurations: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_extraction_hook_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_extraction_hook_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration>>,
    dynamic: KendraDataSource_CustomDocumentEnrichmentConfigurationDynamic,
}

impl KendraDataSource_CustomDocumentEnrichmentConfiguration {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inline_configurations`.\n"]
    pub fn set_inline_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration_InlineConfigurations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `post_extraction_hook_configuration`.\n"]
    pub fn set_post_extraction_hook_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_extraction_hook_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_extraction_hook_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_extraction_hook_configuration`.\n"]
    pub fn set_pre_extraction_hook_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_extraction_hook_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_extraction_hook_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraDataSource_CustomDocumentEnrichmentConfiguration {
    type O = terrars::BlockAssignable<KendraDataSource_CustomDocumentEnrichmentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_CustomDocumentEnrichmentConfiguration {}

impl BuildKendraDataSource_CustomDocumentEnrichmentConfiguration {
    pub fn build(self) -> KendraDataSource_CustomDocumentEnrichmentConfiguration {
        KendraDataSource_CustomDocumentEnrichmentConfiguration {
            role_arn: core::default::Default::default(),
            inline_configurations: core::default::Default::default(),
            post_extraction_hook_configuration: core::default::Default::default(),
            pre_extraction_hook_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_CustomDocumentEnrichmentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_CustomDocumentEnrichmentConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSource_CustomDocumentEnrichmentConfigurationRef {
        KendraDataSource_CustomDocumentEnrichmentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_CustomDocumentEnrichmentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `post_extraction_hook_configuration` after provisioning.\n"]
    pub fn post_extraction_hook_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_CustomDocumentEnrichmentConfiguration_PostExtractionHookConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_extraction_hook_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_extraction_hook_configuration` after provisioning.\n"]
    pub fn pre_extraction_hook_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSource_CustomDocumentEnrichmentConfiguration_PreExtractionHookConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pre_extraction_hook_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSource_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSource_Timeouts {
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

impl terrars::ToListMappable for KendraDataSource_Timeouts {
    type O = terrars::BlockAssignable<KendraDataSource_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSource_Timeouts {}

impl BuildKendraDataSource_Timeouts {
    pub fn build(self) -> KendraDataSource_Timeouts {
        KendraDataSource_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraDataSource_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSource_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraDataSource_TimeoutsRef {
        KendraDataSource_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSource_TimeoutsRef {
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
