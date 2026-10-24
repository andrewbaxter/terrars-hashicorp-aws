use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KendraIndexData {
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
    edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_context_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_units: std::option::Option<std::vec::Vec<KendraIndexCapacityUnitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_metadata_configuration_updates: std
    ::option
    ::Option<std::vec::Vec<KendraIndexDocumentMetadataConfigurationUpdatesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndexServerSideEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KendraIndexTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group_resolution_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndexUserGroupResolutionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_token_configurations: std::option::Option<std::vec::Vec<KendraIndexUserTokenConfigurationsEl>>,
    dynamic: KendraIndexDynamic,
}

struct KendraIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KendraIndexData>,
}

#[derive(Clone)]
pub struct KendraIndex(std::rc::Rc<KendraIndex_>);

impl KendraIndex {
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

    #[doc = "Set the field `edition`.\n"]
    pub fn set_edition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().edition = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `user_context_policy`.\n"]
    pub fn set_user_context_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_context_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_units`.\n"]
    pub fn set_capacity_units(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndexCapacityUnitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_units = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_units = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_metadata_configuration_updates`.\n"]
    pub fn set_document_metadata_configuration_updates(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().document_metadata_configuration_updates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.document_metadata_configuration_updates =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption_configuration`.\n"]
    pub fn set_server_side_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndexServerSideEncryptionConfigurationEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<KendraIndexTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_group_resolution_configuration`.\n"]
    pub fn set_user_group_resolution_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndexUserGroupResolutionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_group_resolution_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_group_resolution_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_token_configurations`.\n"]
    pub fn set_user_token_configurations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndexUserTokenConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_token_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_token_configurations = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> terrars::ListRef<KendraIndexIndexStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_context_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> terrars::ListRef<KendraIndexCapacityUnitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndexServerSideEncryptionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraIndexTimeoutsElRef {
        KendraIndexTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndexUserGroupResolutionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<KendraIndexUserTokenConfigurationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

impl terrars::Referable for KendraIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KendraIndex { }

impl terrars::ToListMappable for KendraIndex {
    type O = terrars::ListRef<KendraIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KendraIndex_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kendra_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKendraIndex {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> KendraIndex {
        let out = KendraIndex(std::rc::Rc::new(KendraIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KendraIndexData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                edition: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_context_policy: core::default::Default::default(),
                capacity_units: core::default::Default::default(),
                document_metadata_configuration_updates: core::default::Default::default(),
                server_side_encryption_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_group_resolution_configuration: core::default::Default::default(),
                user_token_configurations: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KendraIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KendraIndexRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> terrars::ListRef<KendraIndexIndexStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_context_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> terrars::ListRef<KendraIndexCapacityUnitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndexServerSideEncryptionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraIndexTimeoutsElRef {
        KendraIndexTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndexUserGroupResolutionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<KendraIndexUserTokenConfigurationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KendraIndexIndexStatisticsElFaqStatisticsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_question_answers_count: std::option::Option<terrars::PrimField<f64>>,
}

impl KendraIndexIndexStatisticsElFaqStatisticsEl {
    #[doc = "Set the field `indexed_question_answers_count`.\n"]
    pub fn set_indexed_question_answers_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_question_answers_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexIndexStatisticsElFaqStatisticsEl {
    type O = terrars::BlockAssignable<KendraIndexIndexStatisticsElFaqStatisticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexIndexStatisticsElFaqStatisticsEl {}

impl BuildKendraIndexIndexStatisticsElFaqStatisticsEl {
    pub fn build(self) -> KendraIndexIndexStatisticsElFaqStatisticsEl {
        KendraIndexIndexStatisticsElFaqStatisticsEl {
            indexed_question_answers_count: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexIndexStatisticsElFaqStatisticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexIndexStatisticsElFaqStatisticsElRef {
        KendraIndexIndexStatisticsElFaqStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_question_answers_count` after provisioning.\n"]
    pub fn indexed_question_answers_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_question_answers_count", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_documents_count: std::option::Option<terrars::PrimField<f64>>,
}

impl KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[doc = "Set the field `indexed_text_bytes`.\n"]
    pub fn set_indexed_text_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_text_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indexed_text_documents_count`.\n"]
    pub fn set_indexed_text_documents_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_text_documents_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    type O = terrars::BlockAssignable<KendraIndexIndexStatisticsElTextDocumentStatisticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexIndexStatisticsElTextDocumentStatisticsEl {}

impl BuildKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    pub fn build(self) -> KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
        KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
            indexed_text_bytes: core::default::Default::default(),
            indexed_text_documents_count: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
        KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_text_bytes` after provisioning.\n"]
    pub fn indexed_text_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_text_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `indexed_text_documents_count` after provisioning.\n"]
    pub fn indexed_text_documents_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_text_documents_count", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexIndexStatisticsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faq_statistics: std::option::Option<terrars::ListField<KendraIndexIndexStatisticsElFaqStatisticsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_document_statistics: std
    ::option
    ::Option<terrars::ListField<KendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
}

impl KendraIndexIndexStatisticsEl {
    #[doc = "Set the field `faq_statistics`.\n"]
    pub fn set_faq_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<KendraIndexIndexStatisticsElFaqStatisticsEl>>,
    ) -> Self {
        self.faq_statistics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_document_statistics`.\n"]
    pub fn set_text_document_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<KendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
    ) -> Self {
        self.text_document_statistics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexIndexStatisticsEl {
    type O = terrars::BlockAssignable<KendraIndexIndexStatisticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexIndexStatisticsEl {}

impl BuildKendraIndexIndexStatisticsEl {
    pub fn build(self) -> KendraIndexIndexStatisticsEl {
        KendraIndexIndexStatisticsEl {
            faq_statistics: core::default::Default::default(),
            text_document_statistics: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexIndexStatisticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexIndexStatisticsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndexIndexStatisticsElRef {
        KendraIndexIndexStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexIndexStatisticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `faq_statistics` after provisioning.\n"]
    pub fn faq_statistics(&self) -> terrars::ListRef<KendraIndexIndexStatisticsElFaqStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.faq_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `text_document_statistics` after provisioning.\n"]
    pub fn text_document_statistics(&self) -> terrars::ListRef<KendraIndexIndexStatisticsElTextDocumentStatisticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_document_statistics", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexCapacityUnitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl KendraIndexCapacityUnitsEl {
    #[doc = "Set the field `query_capacity_units`.\n"]
    pub fn set_query_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_units`.\n"]
    pub fn set_storage_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_capacity_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexCapacityUnitsEl {
    type O = terrars::BlockAssignable<KendraIndexCapacityUnitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexCapacityUnitsEl {}

impl BuildKendraIndexCapacityUnitsEl {
    pub fn build(self) -> KendraIndexCapacityUnitsEl {
        KendraIndexCapacityUnitsEl {
            query_capacity_units: core::default::Default::default(),
            storage_capacity_units: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexCapacityUnitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexCapacityUnitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndexCapacityUnitsElRef {
        KendraIndexCapacityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexCapacityUnitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_capacity_units` after provisioning.\n"]
    pub fn query_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_units` after provisioning.\n"]
    pub fn storage_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_units", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    freshness: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    importance: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rank_order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values_importance_map: std::option::Option<terrars::RecField<terrars::PrimField<f64>>>,
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `freshness`.\n"]
    pub fn set_freshness(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.freshness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `importance`.\n"]
    pub fn set_importance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.importance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rank_order`.\n"]
    pub fn set_rank_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rank_order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values_importance_map`.\n"]
    pub fn set_values_importance_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.values_importance_map = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    type O = terrars::BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {}

impl BuildKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    pub fn build(self) -> KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
        KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
            duration: core::default::Default::default(),
            freshness: core::default::Default::default(),
            importance: core::default::Default::default(),
            rank_order: core::default::Default::default(),
            values_importance_map: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
        KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `freshness` after provisioning.\n"]
    pub fn freshness(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.freshness", self.base))
    }

    #[doc = "Get a reference to the value of field `importance` after provisioning.\n"]
    pub fn importance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.importance", self.base))
    }

    #[doc = "Get a reference to the value of field `rank_order` after provisioning.\n"]
    pub fn rank_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank_order", self.base))
    }

    #[doc = "Get a reference to the value of field `values_importance_map` after provisioning.\n"]
    pub fn values_importance_map(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.values_importance_map", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    displayable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facetable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    searchable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sortable: std::option::Option<terrars::PrimField<bool>>,
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[doc = "Set the field `displayable`.\n"]
    pub fn set_displayable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.displayable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `facetable`.\n"]
    pub fn set_facetable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.facetable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `searchable`.\n"]
    pub fn set_searchable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.searchable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sortable`.\n"]
    pub fn set_sortable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sortable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    type O = terrars::BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {}

impl BuildKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    pub fn build(self) -> KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
        KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
            displayable: core::default::Default::default(),
            facetable: core::default::Default::default(),
            searchable: core::default::Default::default(),
            sortable: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
        KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `displayable` after provisioning.\n"]
    pub fn displayable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.displayable", self.base))
    }

    #[doc = "Get a reference to the value of field `facetable` after provisioning.\n"]
    pub fn facetable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.facetable", self.base))
    }

    #[doc = "Get a reference to the value of field `searchable` after provisioning.\n"]
    pub fn searchable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.searchable", self.base))
    }

    #[doc = "Get a reference to the value of field `sortable` after provisioning.\n"]
    pub fn sortable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sortable", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraIndexDocumentMetadataConfigurationUpdatesElDynamic {
    relevance: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    search: std::option::Option<terrars::DynamicBlock<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
}

#[derive(Serialize)]
pub struct KendraIndexDocumentMetadataConfigurationUpdatesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevance: std::option::Option<std::vec::Vec<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<std::vec::Vec<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    dynamic: KendraIndexDocumentMetadataConfigurationUpdatesElDynamic,
}

impl KendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[doc = "Set the field `relevance`.\n"]
    pub fn set_relevance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.relevance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.relevance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `search`.\n"]
    pub fn set_search(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.search = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraIndexDocumentMetadataConfigurationUpdatesEl {
    type O = terrars::BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndexDocumentMetadataConfigurationUpdatesEl {
    pub fn build(self) -> KendraIndexDocumentMetadataConfigurationUpdatesEl {
        KendraIndexDocumentMetadataConfigurationUpdatesEl {
            name: self.name,
            type_: self.type_,
            relevance: core::default::Default::default(),
            search: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexDocumentMetadataConfigurationUpdatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexDocumentMetadataConfigurationUpdatesElRef {
        KendraIndexDocumentMetadataConfigurationUpdatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance` after provisioning.\n"]
    pub fn relevance(&self) -> terrars::ListRef<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relevance", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> terrars::ListRef<KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexServerSideEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraIndexServerSideEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexServerSideEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<KendraIndexServerSideEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexServerSideEncryptionConfigurationEl {}

impl BuildKendraIndexServerSideEncryptionConfigurationEl {
    pub fn build(self) -> KendraIndexServerSideEncryptionConfigurationEl {
        KendraIndexServerSideEncryptionConfigurationEl { kms_key_id: core::default::Default::default() }
    }
}

pub struct KendraIndexServerSideEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexServerSideEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexServerSideEncryptionConfigurationElRef {
        KendraIndexServerSideEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexServerSideEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraIndexTimeoutsEl {
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

impl terrars::ToListMappable for KendraIndexTimeoutsEl {
    type O = terrars::BlockAssignable<KendraIndexTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexTimeoutsEl {}

impl BuildKendraIndexTimeoutsEl {
    pub fn build(self) -> KendraIndexTimeoutsEl {
        KendraIndexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndexTimeoutsElRef {
        KendraIndexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexTimeoutsElRef {
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

#[derive(Serialize)]
pub struct KendraIndexUserGroupResolutionConfigurationEl {
    user_group_resolution_mode: terrars::PrimField<std::string::String>,
}

impl KendraIndexUserGroupResolutionConfigurationEl { }

impl terrars::ToListMappable for KendraIndexUserGroupResolutionConfigurationEl {
    type O = terrars::BlockAssignable<KendraIndexUserGroupResolutionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserGroupResolutionConfigurationEl {
    #[doc = ""]
    pub user_group_resolution_mode: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndexUserGroupResolutionConfigurationEl {
    pub fn build(self) -> KendraIndexUserGroupResolutionConfigurationEl {
        KendraIndexUserGroupResolutionConfigurationEl { user_group_resolution_mode: self.user_group_resolution_mode }
    }
}

pub struct KendraIndexUserGroupResolutionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexUserGroupResolutionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexUserGroupResolutionConfigurationElRef {
        KendraIndexUserGroupResolutionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserGroupResolutionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_mode` after provisioning.\n"]
    pub fn user_group_resolution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_resolution_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    group_attribute_field: terrars::PrimField<std::string::String>,
    user_name_attribute_field: terrars::PrimField<std::string::String>,
}

impl KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl { }

impl terrars::ToListMappable for KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    type O = terrars::BlockAssignable<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    #[doc = ""]
    pub group_attribute_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_name_attribute_field: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    pub fn build(self) -> KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
        KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
            group_attribute_field: self.group_attribute_field,
            user_name_attribute_field: self.user_name_attribute_field,
        }
    }
}

pub struct KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
        KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute_field", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name_attribute_field", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    key_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_name_attribute_field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[doc = "Set the field `claim_regex`.\n"]
    pub fn set_claim_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.claim_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_attribute_field`.\n"]
    pub fn set_group_attribute_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.group_attribute_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_name_attribute_field`.\n"]
    pub fn set_user_name_attribute_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_name_attribute_field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    type O = terrars::BlockAssignable<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[doc = ""]
    pub key_location: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    pub fn build(self) -> KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
        KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
            claim_regex: core::default::Default::default(),
            group_attribute_field: core::default::Default::default(),
            issuer: core::default::Default::default(),
            key_location: self.key_location,
            secrets_manager_arn: core::default::Default::default(),
            url: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
        KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_regex` after provisioning.\n"]
    pub fn claim_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute_field", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `key_location` after provisioning.\n"]
    pub fn key_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_location", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name_attribute_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraIndexUserTokenConfigurationsElDynamic {
    json_token_type_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    jwt_token_type_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KendraIndexUserTokenConfigurationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_token_type_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_token_type_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
    dynamic: KendraIndexUserTokenConfigurationsElDynamic,
}

impl KendraIndexUserTokenConfigurationsEl {
    #[doc = "Set the field `json_token_type_configuration`.\n"]
    pub fn set_json_token_type_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_token_type_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_token_type_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jwt_token_type_configuration`.\n"]
    pub fn set_jwt_token_type_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jwt_token_type_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_token_type_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraIndexUserTokenConfigurationsEl {
    type O = terrars::BlockAssignable<KendraIndexUserTokenConfigurationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserTokenConfigurationsEl {}

impl BuildKendraIndexUserTokenConfigurationsEl {
    pub fn build(self) -> KendraIndexUserTokenConfigurationsEl {
        KendraIndexUserTokenConfigurationsEl {
            json_token_type_configuration: core::default::Default::default(),
            jwt_token_type_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexUserTokenConfigurationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndexUserTokenConfigurationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndexUserTokenConfigurationsElRef {
        KendraIndexUserTokenConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserTokenConfigurationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_token_type_configuration` after provisioning.\n"]
    pub fn json_token_type_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_token_type_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token_type_configuration` after provisioning.\n"]
    pub fn jwt_token_type_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_token_type_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraIndexDynamic {
    capacity_units: std::option::Option<terrars::DynamicBlock<KendraIndexCapacityUnitsEl>>,
    document_metadata_configuration_updates: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndexDocumentMetadataConfigurationUpdatesEl>>,
    server_side_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndexServerSideEncryptionConfigurationEl>>,
    user_group_resolution_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndexUserGroupResolutionConfigurationEl>>,
    user_token_configurations: std::option::Option<terrars::DynamicBlock<KendraIndexUserTokenConfigurationsEl>>,
}
