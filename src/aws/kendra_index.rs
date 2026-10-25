#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KendraIndexData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_context_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_units: std::option::Option<std::vec::Vec<KendraIndex_CapacityUnits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_metadata_configuration_updates: std
    ::option
    ::Option<std::vec::Vec<KendraIndex_DocumentMetadataConfigurationUpdates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndex_ServerSideEncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KendraIndex_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group_resolution_configuration: std::option::Option<std::vec::Vec<KendraIndex_UserGroupResolutionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_token_configurations: std::option::Option<std::vec::Vec<KendraIndex_UserTokenConfigurations>>,
    dynamic: KendraIndexDynamic,
}

struct KendraIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KendraIndexData_>,
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

    #[doc = "Set the field `user_context_policy`.\n"]
    pub fn set_user_context_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_context_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_units`.\n"]
    pub fn set_capacity_units(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndex_CapacityUnits>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndex_DocumentMetadataConfigurationUpdates>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndex_ServerSideEncryptionConfiguration>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<KendraIndex_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_group_resolution_configuration`.\n"]
    pub fn set_user_group_resolution_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndex_UserGroupResolutionConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KendraIndex_UserTokenConfigurations>>,
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
    pub fn index_statistics(&self) -> terrars::ListRef<KendraIndex_IndexStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
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
    pub fn capacity_units(&self) -> terrars::ListRef<KendraIndex_CapacityUnitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndex_ServerSideEncryptionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraIndex_TimeoutsRef {
        KendraIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndex_UserGroupResolutionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<KendraIndex_UserTokenConfigurationsRef> {
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
            data: std::cell::RefCell::new(KendraIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                edition: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
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
    pub fn index_statistics(&self) -> terrars::ListRef<KendraIndex_IndexStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
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
    pub fn capacity_units(&self) -> terrars::ListRef<KendraIndex_CapacityUnitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndex_ServerSideEncryptionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraIndex_TimeoutsRef {
        KendraIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndex_UserGroupResolutionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> terrars::ListRef<KendraIndex_UserTokenConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct KendraIndexDynamic {
    capacity_units: std::option::Option<terrars::DynamicBlock<KendraIndex_CapacityUnits>>,
    document_metadata_configuration_updates: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndex_DocumentMetadataConfigurationUpdates>>,
    server_side_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndex_ServerSideEncryptionConfiguration>>,
    user_group_resolution_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndex_UserGroupResolutionConfiguration>>,
    user_token_configurations: std::option::Option<terrars::DynamicBlock<KendraIndex_UserTokenConfigurations>>,
}

#[derive(Serialize)]
pub struct KendraIndex_IndexStatistics_FaqStatistics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_question_answers_count: std::option::Option<terrars::PrimField<f64>>,
}

impl KendraIndex_IndexStatistics_FaqStatistics {
    #[doc = "Set the field `indexed_question_answers_count`.\n"]
    pub fn set_indexed_question_answers_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.indexed_question_answers_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndex_IndexStatistics_FaqStatistics {
    type O = terrars::BlockAssignable<KendraIndex_IndexStatistics_FaqStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_IndexStatistics_FaqStatistics {}

impl BuildKendraIndex_IndexStatistics_FaqStatistics {
    pub fn build(self) -> KendraIndex_IndexStatistics_FaqStatistics {
        KendraIndex_IndexStatistics_FaqStatistics {
            indexed_question_answers_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_IndexStatistics_FaqStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_IndexStatistics_FaqStatisticsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndex_IndexStatistics_FaqStatisticsRef {
        KendraIndex_IndexStatistics_FaqStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_IndexStatistics_FaqStatisticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_question_answers_count` after provisioning.\n"]
    pub fn indexed_question_answers_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_question_answers_count", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndex_IndexStatistics_TextDocumentStatistics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_text_documents_count: std::option::Option<terrars::PrimField<f64>>,
}

impl KendraIndex_IndexStatistics_TextDocumentStatistics {
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

impl terrars::ToListMappable for KendraIndex_IndexStatistics_TextDocumentStatistics {
    type O = terrars::BlockAssignable<KendraIndex_IndexStatistics_TextDocumentStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_IndexStatistics_TextDocumentStatistics {}

impl BuildKendraIndex_IndexStatistics_TextDocumentStatistics {
    pub fn build(self) -> KendraIndex_IndexStatistics_TextDocumentStatistics {
        KendraIndex_IndexStatistics_TextDocumentStatistics {
            indexed_text_bytes: core::default::Default::default(),
            indexed_text_documents_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_IndexStatistics_TextDocumentStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_IndexStatistics_TextDocumentStatisticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_IndexStatistics_TextDocumentStatisticsRef {
        KendraIndex_IndexStatistics_TextDocumentStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_IndexStatistics_TextDocumentStatisticsRef {
    #[allow(dead_code)]
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
pub struct KendraIndex_IndexStatistics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faq_statistics: std::option::Option<terrars::ListField<KendraIndex_IndexStatistics_FaqStatistics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_document_statistics: std
    ::option
    ::Option<terrars::ListField<KendraIndex_IndexStatistics_TextDocumentStatistics>>,
}

impl KendraIndex_IndexStatistics {
    #[doc = "Set the field `faq_statistics`.\n"]
    pub fn set_faq_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<KendraIndex_IndexStatistics_FaqStatistics>>,
    ) -> Self {
        self.faq_statistics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_document_statistics`.\n"]
    pub fn set_text_document_statistics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<KendraIndex_IndexStatistics_TextDocumentStatistics>>,
    ) -> Self {
        self.text_document_statistics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndex_IndexStatistics {
    type O = terrars::BlockAssignable<KendraIndex_IndexStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_IndexStatistics {}

impl BuildKendraIndex_IndexStatistics {
    pub fn build(self) -> KendraIndex_IndexStatistics {
        KendraIndex_IndexStatistics {
            faq_statistics: core::default::Default::default(),
            text_document_statistics: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_IndexStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_IndexStatisticsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndex_IndexStatisticsRef {
        KendraIndex_IndexStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_IndexStatisticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `faq_statistics` after provisioning.\n"]
    pub fn faq_statistics(&self) -> terrars::ListRef<KendraIndex_IndexStatistics_FaqStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.faq_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `text_document_statistics` after provisioning.\n"]
    pub fn text_document_statistics(&self) -> terrars::ListRef<KendraIndex_IndexStatistics_TextDocumentStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_document_statistics", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndex_CapacityUnits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl KendraIndex_CapacityUnits {
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

impl terrars::ToListMappable for KendraIndex_CapacityUnits {
    type O = terrars::BlockAssignable<KendraIndex_CapacityUnits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_CapacityUnits {}

impl BuildKendraIndex_CapacityUnits {
    pub fn build(self) -> KendraIndex_CapacityUnits {
        KendraIndex_CapacityUnits {
            query_capacity_units: core::default::Default::default(),
            storage_capacity_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_CapacityUnitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_CapacityUnitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndex_CapacityUnitsRef {
        KendraIndex_CapacityUnitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_CapacityUnitsRef {
    #[allow(dead_code)]
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
pub struct KendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
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

impl KendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
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

impl terrars::ToListMappable for KendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
    type O = terrars::BlockAssignable<KendraIndex_DocumentMetadataConfigurationUpdates_Relevance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {}

impl BuildKendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
    pub fn build(self) -> KendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
        KendraIndex_DocumentMetadataConfigurationUpdates_Relevance {
            duration: core::default::Default::default(),
            freshness: core::default::Default::default(),
            importance: core::default::Default::default(),
            rank_order: core::default::Default::default(),
            values_importance_map: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
        KendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef {
    #[allow(dead_code)]
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
pub struct KendraIndex_DocumentMetadataConfigurationUpdates_Search {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    displayable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facetable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    searchable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sortable: std::option::Option<terrars::PrimField<bool>>,
}

impl KendraIndex_DocumentMetadataConfigurationUpdates_Search {
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

impl terrars::ToListMappable for KendraIndex_DocumentMetadataConfigurationUpdates_Search {
    type O = terrars::BlockAssignable<KendraIndex_DocumentMetadataConfigurationUpdates_Search>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_DocumentMetadataConfigurationUpdates_Search {}

impl BuildKendraIndex_DocumentMetadataConfigurationUpdates_Search {
    pub fn build(self) -> KendraIndex_DocumentMetadataConfigurationUpdates_Search {
        KendraIndex_DocumentMetadataConfigurationUpdates_Search {
            displayable: core::default::Default::default(),
            facetable: core::default::Default::default(),
            searchable: core::default::Default::default(),
            sortable: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
        KendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_DocumentMetadataConfigurationUpdates_SearchRef {
    #[allow(dead_code)]
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
struct KendraIndex_DocumentMetadataConfigurationUpdatesDynamic {
    relevance: std::option::Option<terrars::DynamicBlock<KendraIndex_DocumentMetadataConfigurationUpdates_Relevance>>,
    search: std::option::Option<terrars::DynamicBlock<KendraIndex_DocumentMetadataConfigurationUpdates_Search>>,
}

#[derive(Serialize)]
pub struct KendraIndex_DocumentMetadataConfigurationUpdates {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relevance: std::option::Option<std::vec::Vec<KendraIndex_DocumentMetadataConfigurationUpdates_Relevance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<std::vec::Vec<KendraIndex_DocumentMetadataConfigurationUpdates_Search>>,
    dynamic: KendraIndex_DocumentMetadataConfigurationUpdatesDynamic,
}

impl KendraIndex_DocumentMetadataConfigurationUpdates {
    #[doc = "Set the field `relevance`.\n"]
    pub fn set_relevance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraIndex_DocumentMetadataConfigurationUpdates_Relevance>>,
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
                    ::Into<terrars::BlockAssignable<KendraIndex_DocumentMetadataConfigurationUpdates_Search>>,
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

impl terrars::ToListMappable for KendraIndex_DocumentMetadataConfigurationUpdates {
    type O = terrars::BlockAssignable<KendraIndex_DocumentMetadataConfigurationUpdates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_DocumentMetadataConfigurationUpdates {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndex_DocumentMetadataConfigurationUpdates {
    pub fn build(self) -> KendraIndex_DocumentMetadataConfigurationUpdates {
        KendraIndex_DocumentMetadataConfigurationUpdates {
            name: self.name,
            type_: self.type_,
            relevance: core::default::Default::default(),
            search: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_DocumentMetadataConfigurationUpdatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_DocumentMetadataConfigurationUpdatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_DocumentMetadataConfigurationUpdatesRef {
        KendraIndex_DocumentMetadataConfigurationUpdatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_DocumentMetadataConfigurationUpdatesRef {
    #[allow(dead_code)]
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
    pub fn relevance(&self) -> terrars::ListRef<KendraIndex_DocumentMetadataConfigurationUpdates_RelevanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relevance", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> terrars::ListRef<KendraIndex_DocumentMetadataConfigurationUpdates_SearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndex_ServerSideEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraIndex_ServerSideEncryptionConfiguration {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraIndex_ServerSideEncryptionConfiguration {
    type O = terrars::BlockAssignable<KendraIndex_ServerSideEncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_ServerSideEncryptionConfiguration {}

impl BuildKendraIndex_ServerSideEncryptionConfiguration {
    pub fn build(self) -> KendraIndex_ServerSideEncryptionConfiguration {
        KendraIndex_ServerSideEncryptionConfiguration { kms_key_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_ServerSideEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_ServerSideEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_ServerSideEncryptionConfigurationRef {
        KendraIndex_ServerSideEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_ServerSideEncryptionConfigurationRef {
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
pub struct KendraIndex_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraIndex_Timeouts {
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

impl terrars::ToListMappable for KendraIndex_Timeouts {
    type O = terrars::BlockAssignable<KendraIndex_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_Timeouts {}

impl BuildKendraIndex_Timeouts {
    pub fn build(self) -> KendraIndex_Timeouts {
        KendraIndex_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndex_TimeoutsRef {
        KendraIndex_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_TimeoutsRef {
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

#[derive(Serialize)]
pub struct KendraIndex_UserGroupResolutionConfiguration {
    user_group_resolution_mode: terrars::PrimField<std::string::String>,
}

impl KendraIndex_UserGroupResolutionConfiguration { }

impl terrars::ToListMappable for KendraIndex_UserGroupResolutionConfiguration {
    type O = terrars::BlockAssignable<KendraIndex_UserGroupResolutionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_UserGroupResolutionConfiguration {
    #[doc = ""]
    pub user_group_resolution_mode: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndex_UserGroupResolutionConfiguration {
    pub fn build(self) -> KendraIndex_UserGroupResolutionConfiguration {
        KendraIndex_UserGroupResolutionConfiguration { user_group_resolution_mode: self.user_group_resolution_mode }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_UserGroupResolutionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_UserGroupResolutionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_UserGroupResolutionConfigurationRef {
        KendraIndex_UserGroupResolutionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_UserGroupResolutionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_mode` after provisioning.\n"]
    pub fn user_group_resolution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_resolution_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    group_attribute_field: terrars::PrimField<std::string::String>,
    user_name_attribute_field: terrars::PrimField<std::string::String>,
}

impl KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration { }

impl terrars::ToListMappable for KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    type O = terrars::BlockAssignable<KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    #[doc = ""]
    pub group_attribute_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_name_attribute_field: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
    pub fn build(self) -> KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
        KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration {
            group_attribute_field: self.group_attribute_field,
            user_name_attribute_field: self.user_name_attribute_field,
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
        KendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef {
    #[allow(dead_code)]
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
pub struct KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
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

impl KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
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

impl terrars::ToListMappable for KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
    type O = terrars::BlockAssignable<KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
    #[doc = ""]
    pub key_location: terrars::PrimField<std::string::String>,
}

impl BuildKendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
    pub fn build(self) -> KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
        KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration {
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

#[allow(dead_code)]
pub struct KendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
        KendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef {
    #[allow(dead_code)]
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
struct KendraIndex_UserTokenConfigurationsDynamic {
    json_token_type_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>>,
    jwt_token_type_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>>,
}

#[derive(Serialize)]
pub struct KendraIndex_UserTokenConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_token_type_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_token_type_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>>,
    dynamic: KendraIndex_UserTokenConfigurationsDynamic,
}

impl KendraIndex_UserTokenConfigurations {
    #[doc = "Set the field `json_token_type_configuration`.\n"]
    pub fn set_json_token_type_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraIndex_UserTokenConfigurations_JsonTokenTypeConfiguration>>,
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
                    ::Into<terrars::BlockAssignable<KendraIndex_UserTokenConfigurations_JwtTokenTypeConfiguration>>,
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

impl terrars::ToListMappable for KendraIndex_UserTokenConfigurations {
    type O = terrars::BlockAssignable<KendraIndex_UserTokenConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndex_UserTokenConfigurations {}

impl BuildKendraIndex_UserTokenConfigurations {
    pub fn build(self) -> KendraIndex_UserTokenConfigurations {
        KendraIndex_UserTokenConfigurations {
            json_token_type_configuration: core::default::Default::default(),
            jwt_token_type_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KendraIndex_UserTokenConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraIndex_UserTokenConfigurationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraIndex_UserTokenConfigurationsRef {
        KendraIndex_UserTokenConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndex_UserTokenConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_token_type_configuration` after provisioning.\n"]
    pub fn json_token_type_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndex_UserTokenConfigurations_JsonTokenTypeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_token_type_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token_type_configuration` after provisioning.\n"]
    pub fn jwt_token_type_configuration(
        &self,
    ) -> terrars::ListRef<KendraIndex_UserTokenConfigurations_JwtTokenTypeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_token_type_configuration", self.base))
    }
}
