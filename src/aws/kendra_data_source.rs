use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KendraDataSourceData {
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
    configuration: std::option::Option<std::vec::Vec<KendraDataSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_document_enrichment_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KendraDataSourceTimeoutsEl>,
    dynamic: KendraDataSourceDynamic,
}

struct KendraDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KendraDataSourceData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSourceConfigurationEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<KendraDataSourceTimeoutsEl>) -> Self {
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
    pub fn configuration(&self) -> terrars::ListRef<KendraDataSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_document_enrichment_configuration` after provisioning.\n"]
    pub fn custom_document_enrichment_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.custom_document_enrichment_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraDataSourceTimeoutsElRef {
        KendraDataSourceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(KendraDataSourceData {
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
    pub fn configuration(&self) -> terrars::ListRef<KendraDataSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_document_enrichment_configuration` after provisioning.\n"]
    pub fn custom_document_enrichment_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.custom_document_enrichment_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraDataSourceTimeoutsElRef {
        KendraDataSourceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    #[doc = "Set the field `key_path`.\n"]
    pub fn set_key_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    type O =
        terrars::BlockAssignable<KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {}

impl BuildKendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
        KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
            key_path: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
        KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_path` after provisioning.\n"]
    pub fn key_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_path", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    type O =
        terrars::BlockAssignable<KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {}

impl BuildKendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
        KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
            s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
        KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElS3ConfigurationElDynamic {
    access_control_list_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl>>,
    documents_metadata_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElS3ConfigurationEl {
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
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_metadata_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl>>,
    dynamic: KendraDataSourceConfigurationElS3ConfigurationElDynamic,
}

impl KendraDataSourceConfigurationElS3ConfigurationEl {
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
                            KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl,
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
                            KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl,
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

impl terrars::ToListMappable for KendraDataSourceConfigurationElS3ConfigurationEl {
    type O = terrars::BlockAssignable<KendraDataSourceConfigurationElS3ConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElS3ConfigurationEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSourceConfigurationElS3ConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElS3ConfigurationEl {
        KendraDataSourceConfigurationElS3ConfigurationEl {
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

pub struct KendraDataSourceConfigurationElS3ConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElS3ConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElRef {
        KendraDataSourceConfigurationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElS3ConfigurationElRef {
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
    ) -> terrars::ListRef<KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_list_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `documents_metadata_configuration` after provisioning.\n"]
    pub fn documents_metadata_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documents_metadata_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl {
    credentials: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl { }

impl terrars
::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl {
    #[doc = ""]
    pub credentials: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl {
            credentials: self.credentials,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef {
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
struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElDynamic {
    basic_authentication: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_authentication: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl,
        >,
    >,
    dynamic: KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElDynamic,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
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
                            KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl,
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

impl terrars::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
            basic_authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    type O = terrars::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
            credentials: core::default::Default::default(),
            host: self.host,
            port: self.port,
        }
    }
}

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
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
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    seed_urls: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    #[doc = "Set the field `web_crawler_mode`.\n"]
    pub fn set_web_crawler_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_crawler_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    #[doc = ""]
    pub seed_urls: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
            seed_urls: self.seed_urls,
            web_crawler_mode: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
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
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
    site_maps: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl { }

impl terrars::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
    #[doc = ""]
    pub site_maps: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
            site_maps: self.site_maps,
        }
    }
}

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_maps` after provisioning.\n"]
    pub fn site_maps(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.site_maps", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElDynamic {
    seed_url_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl>,
    >,
    site_maps_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seed_url_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    site_maps_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl>>,
    dynamic: KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElDynamic,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
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
                            KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl,
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
                            KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl,
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

impl terrars::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    type O = terrars::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
            seed_url_configuration: core::default::Default::default(),
            site_maps_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed_url_configuration` after provisioning.\n"]
    pub fn seed_url_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.seed_url_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `site_maps_configuration` after provisioning.\n"]
    pub fn site_maps_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.site_maps_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElWebCrawlerConfigurationElDynamic {
    authentication_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl>,
    >,
    proxy_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl>>,
    urls: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>>,
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
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
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    urls: std::option::Option<std::vec::Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>>,
    dynamic: KendraDataSourceConfigurationElWebCrawlerConfigurationElDynamic,
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
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
                            KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl,
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
                        ::BlockAssignable<
                            KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl,
                        >,
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
                    ::Into<terrars::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>>,
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

impl terrars::ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
    type O = terrars::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationEl {}

impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
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

pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
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
    ) -> terrars::ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `urls` after provisioning.\n"]
    pub fn urls(&self) -> terrars::ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElDynamic {
    s3_configuration: std::option::Option<terrars::DynamicBlock<KendraDataSourceConfigurationElS3ConfigurationEl>>,
    web_crawler_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KendraDataSourceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std::option::Option<std::vec::Vec<KendraDataSourceConfigurationElS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>>,
    dynamic: KendraDataSourceConfigurationElDynamic,
}

impl KendraDataSourceConfigurationEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraDataSourceConfigurationElS3ConfigurationEl>>,
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

    #[doc = "Set the field `web_crawler_configuration`.\n"]
    pub fn set_web_crawler_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>>,
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

impl terrars::ToListMappable for KendraDataSourceConfigurationEl {
    type O = terrars::BlockAssignable<KendraDataSourceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceConfigurationEl {}

impl BuildKendraDataSourceConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationEl {
        KendraDataSourceConfigurationEl {
            s3_configuration: core::default::Default::default(),
            web_crawler_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraDataSourceConfigurationElRef {
        KendraDataSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> terrars::ListRef<KendraDataSourceConfigurationElS3ConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `web_crawler_configuration` after provisioning.\n"]
    pub fn web_crawler_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_crawler_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl {
            date_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef {
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
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElDynamic {
    condition_on_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
    condition_document_attribute_key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_on_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl,
        >,
    >,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl,
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
    #[doc = ""]
    pub condition_document_attribute_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
            condition_document_attribute_key: self.condition_document_attribute_key,
            operator: self.operator,
            condition_on_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef {
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
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_on_value", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl {
            date_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef {
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
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElDynamic {
    target_document_attribute_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
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
            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl,
        >,
    >,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl,
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

impl terrars::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
            target_document_attribute_key: core::default::Default::default(),
            target_document_attribute_value_deletion: core::default::Default::default(),
            target_document_attribute_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
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
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_document_attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElDynamic {
    condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl>,
    >,
    target: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_content_deletion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl>>,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl,
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl,
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

impl terrars::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
    type O = terrars::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
            document_content_deletion: core::default::Default::default(),
            condition: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
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
    ) -> terrars
    ::ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(
        &self,
    ) -> terrars::ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
            date_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
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
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElDynamic {
    condition_on_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
    condition_document_attribute_key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_on_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
        >,
    >,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
    #[doc = ""]
    pub condition_document_attribute_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl {
            condition_document_attribute_key: self.condition_document_attribute_key,
            operator: self.operator,
            condition_on_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef {
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
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_on_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElDynamic {
    invocation_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
    lambda_arn: terrars::PrimField<std::string::String>,
    s3_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl,
        >,
    >,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl,
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
            lambda_arn: self.lambda_arn,
            s3_bucket: self.s3_bucket,
            invocation_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef {
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
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list_value: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl {
            date_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef {
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
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElDynamic {
    condition_on_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
    condition_document_attribute_key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_on_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
        >,
    >,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl,
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
    type O =
        terrars
        ::BlockAssignable<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
    #[doc = ""]
    pub condition_document_attribute_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl {
            condition_document_attribute_key: self.condition_document_attribute_key,
            operator: self.operator,
            condition_on_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef {
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
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_on_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElDynamic {
    invocation_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
    lambda_arn: terrars::PrimField<std::string::String>,
    s3_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl,
        >,
    >,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl,
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
::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
            lambda_arn: self.lambda_arn,
            s3_bucket: self.s3_bucket,
            invocation_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
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
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElDynamic {
    inline_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl>>,
    post_extraction_hook_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl>,
    >,
    pre_extraction_hook_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_configurations: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_extraction_hook_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_extraction_hook_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl>>,
    dynamic: KendraDataSourceCustomDocumentEnrichmentConfigurationElDynamic,
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
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
                        ::BlockAssignable<
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl,
                        >,
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl,
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
                            KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl,
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

impl terrars::ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
    type O = terrars::BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationEl {}

impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
            role_arn: core::default::Default::default(),
            inline_configurations: core::default::Default::default(),
            post_extraction_hook_configuration: core::default::Default::default(),
            pre_extraction_hook_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
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
    ) -> terrars
    ::ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_extraction_hook_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_extraction_hook_configuration` after provisioning.\n"]
    pub fn pre_extraction_hook_configuration(
        &self,
    ) -> terrars::ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pre_extraction_hook_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraDataSourceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraDataSourceTimeoutsEl {
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

impl terrars::ToListMappable for KendraDataSourceTimeoutsEl {
    type O = terrars::BlockAssignable<KendraDataSourceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraDataSourceTimeoutsEl {}

impl BuildKendraDataSourceTimeoutsEl {
    pub fn build(self) -> KendraDataSourceTimeoutsEl {
        KendraDataSourceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KendraDataSourceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraDataSourceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraDataSourceTimeoutsElRef {
        KendraDataSourceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraDataSourceTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct KendraDataSourceDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<KendraDataSourceConfigurationEl>>,
    custom_document_enrichment_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>>,
}
