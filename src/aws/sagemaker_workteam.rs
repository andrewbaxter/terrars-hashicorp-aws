#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerWorkteamData_ {
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
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workforce_name: std::option::Option<terrars::PrimField<std::string::String>>,
    workteam_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member_definition: std::option::Option<std::vec::Vec<SagemakerWorkteam_MemberDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_configuration: std::option::Option<std::vec::Vec<SagemakerWorkteam_NotificationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_access_configuration: std::option::Option<std::vec::Vec<SagemakerWorkteam_WorkerAccessConfiguration>>,
    dynamic: SagemakerWorkteamDynamic,
}

struct SagemakerWorkteam_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerWorkteamData_>,
}

#[derive(Clone)]
pub struct SagemakerWorkteam(std::rc::Rc<SagemakerWorkteam_>);

impl SagemakerWorkteam {
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

    #[doc = "Set the field `workforce_name`.\n"]
    pub fn set_workforce_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().workforce_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `member_definition`.\n"]
    pub fn set_member_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkteam_MemberDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().member_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.member_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_configuration`.\n"]
    pub fn set_notification_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkteam_NotificationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker_access_configuration`.\n"]
    pub fn set_worker_access_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkteam_WorkerAccessConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_access_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_access_configuration = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workteam_name` after provisioning.\n"]
    pub fn workteam_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workteam_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_definition` after provisioning.\n"]
    pub fn member_definition(&self) -> terrars::ListRef<SagemakerWorkteam_MemberDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(&self) -> terrars::ListRef<SagemakerWorkteam_NotificationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_access_configuration` after provisioning.\n"]
    pub fn worker_access_configuration(&self) -> terrars::ListRef<SagemakerWorkteam_WorkerAccessConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_access_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerWorkteam {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerWorkteam { }

impl terrars::ToListMappable for SagemakerWorkteam {
    type O = terrars::ListRef<SagemakerWorkteamRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerWorkteam_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_workteam".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerWorkteam {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workteam_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerWorkteam {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerWorkteam {
        let out = SagemakerWorkteam(std::rc::Rc::new(SagemakerWorkteam_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerWorkteamData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                workforce_name: core::default::Default::default(),
                workteam_name: self.workteam_name,
                member_definition: core::default::Default::default(),
                notification_configuration: core::default::Default::default(),
                worker_access_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerWorkteamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerWorkteamRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workteam_name` after provisioning.\n"]
    pub fn workteam_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workteam_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_definition` after provisioning.\n"]
    pub fn member_definition(&self) -> terrars::ListRef<SagemakerWorkteam_MemberDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(&self) -> terrars::ListRef<SagemakerWorkteam_NotificationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_access_configuration` after provisioning.\n"]
    pub fn worker_access_configuration(&self) -> terrars::ListRef<SagemakerWorkteam_WorkerAccessConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_access_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteamDynamic {
    member_definition: std::option::Option<terrars::DynamicBlock<SagemakerWorkteam_MemberDefinition>>,
    notification_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerWorkteam_NotificationConfiguration>>,
    worker_access_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerWorkteam_WorkerAccessConfiguration>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition {
    client_id: terrars::PrimField<std::string::String>,
    user_group: terrars::PrimField<std::string::String>,
    user_pool: terrars::PrimField<std::string::String>,
}

impl SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition { }

impl terrars::ToListMappable for SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition {
    type O = terrars::BlockAssignable<SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_MemberDefinition_CognitoMemberDefinition {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_group: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerWorkteam_MemberDefinition_CognitoMemberDefinition {
    pub fn build(self) -> SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition {
        SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition {
            client_id: self.client_id,
            user_group: self.user_group,
            user_pool: self.user_pool,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_MemberDefinition_CognitoMemberDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_MemberDefinition_CognitoMemberDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerWorkteam_MemberDefinition_CognitoMemberDefinitionRef {
        SagemakerWorkteam_MemberDefinition_CognitoMemberDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_MemberDefinition_CognitoMemberDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_group` after provisioning.\n"]
    pub fn user_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool` after provisioning.\n"]
    pub fn user_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_MemberDefinition_OidcMemberDefinition {
    groups: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerWorkteam_MemberDefinition_OidcMemberDefinition { }

impl terrars::ToListMappable for SagemakerWorkteam_MemberDefinition_OidcMemberDefinition {
    type O = terrars::BlockAssignable<SagemakerWorkteam_MemberDefinition_OidcMemberDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_MemberDefinition_OidcMemberDefinition {
    #[doc = ""]
    pub groups: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerWorkteam_MemberDefinition_OidcMemberDefinition {
    pub fn build(self) -> SagemakerWorkteam_MemberDefinition_OidcMemberDefinition {
        SagemakerWorkteam_MemberDefinition_OidcMemberDefinition { groups: self.groups }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_MemberDefinition_OidcMemberDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_MemberDefinition_OidcMemberDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerWorkteam_MemberDefinition_OidcMemberDefinitionRef {
        SagemakerWorkteam_MemberDefinition_OidcMemberDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_MemberDefinition_OidcMemberDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteam_MemberDefinitionDynamic {
    cognito_member_definition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition>>,
    oidc_member_definition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerWorkteam_MemberDefinition_OidcMemberDefinition>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_MemberDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_member_definition: std
    ::option
    ::Option<std::vec::Vec<SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_member_definition: std::option::Option<std::vec::Vec<SagemakerWorkteam_MemberDefinition_OidcMemberDefinition>>,
    dynamic: SagemakerWorkteam_MemberDefinitionDynamic,
}

impl SagemakerWorkteam_MemberDefinition {
    #[doc = "Set the field `cognito_member_definition`.\n"]
    pub fn set_cognito_member_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerWorkteam_MemberDefinition_CognitoMemberDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cognito_member_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_member_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_member_definition`.\n"]
    pub fn set_oidc_member_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerWorkteam_MemberDefinition_OidcMemberDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oidc_member_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_member_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkteam_MemberDefinition {
    type O = terrars::BlockAssignable<SagemakerWorkteam_MemberDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_MemberDefinition {}

impl BuildSagemakerWorkteam_MemberDefinition {
    pub fn build(self) -> SagemakerWorkteam_MemberDefinition {
        SagemakerWorkteam_MemberDefinition {
            cognito_member_definition: core::default::Default::default(),
            oidc_member_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_MemberDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_MemberDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerWorkteam_MemberDefinitionRef {
        SagemakerWorkteam_MemberDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_MemberDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cognito_member_definition` after provisioning.\n"]
    pub fn cognito_member_definition(
        &self,
    ) -> terrars::ListRef<SagemakerWorkteam_MemberDefinition_CognitoMemberDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_member_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_member_definition` after provisioning.\n"]
    pub fn oidc_member_definition(&self) -> terrars::ListRef<SagemakerWorkteam_MemberDefinition_OidcMemberDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_member_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_NotificationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerWorkteam_NotificationConfiguration {
    #[doc = "Set the field `notification_topic_arn`.\n"]
    pub fn set_notification_topic_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.notification_topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkteam_NotificationConfiguration {
    type O = terrars::BlockAssignable<SagemakerWorkteam_NotificationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_NotificationConfiguration {}

impl BuildSagemakerWorkteam_NotificationConfiguration {
    pub fn build(self) -> SagemakerWorkteam_NotificationConfiguration {
        SagemakerWorkteam_NotificationConfiguration { notification_topic_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_NotificationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_NotificationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerWorkteam_NotificationConfigurationRef {
        SagemakerWorkteam_NotificationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_NotificationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notification_topic_arn` after provisioning.\n"]
    pub fn notification_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_source_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {
    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_source_ip`.\n"]
    pub fn set_vpc_source_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_source_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {
    type O = terrars::BlockAssignable<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {}

impl BuildSagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {
    pub fn build(self) -> SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {
        SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints {
            source_ip: core::default::Default::default(),
            vpc_source_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraintsRef {
        SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_source_ip` after provisioning.\n"]
    pub fn vpc_source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_source_ip", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteam_WorkerAccessConfiguration_S3PresignDynamic {
    iam_policy_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_WorkerAccessConfiguration_S3Presign {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_policy_constraints: std
    ::option
    ::Option<std::vec::Vec<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints>>,
    dynamic: SagemakerWorkteam_WorkerAccessConfiguration_S3PresignDynamic,
}

impl SagemakerWorkteam_WorkerAccessConfiguration_S3Presign {
    #[doc = "Set the field `iam_policy_constraints`.\n"]
    pub fn set_iam_policy_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraints>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iam_policy_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_policy_constraints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkteam_WorkerAccessConfiguration_S3Presign {
    type O = terrars::BlockAssignable<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_WorkerAccessConfiguration_S3Presign {}

impl BuildSagemakerWorkteam_WorkerAccessConfiguration_S3Presign {
    pub fn build(self) -> SagemakerWorkteam_WorkerAccessConfiguration_S3Presign {
        SagemakerWorkteam_WorkerAccessConfiguration_S3Presign {
            iam_policy_constraints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_WorkerAccessConfiguration_S3PresignRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_WorkerAccessConfiguration_S3PresignRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerWorkteam_WorkerAccessConfiguration_S3PresignRef {
        SagemakerWorkteam_WorkerAccessConfiguration_S3PresignRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_WorkerAccessConfiguration_S3PresignRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_policy_constraints` after provisioning.\n"]
    pub fn iam_policy_constraints(
        &self,
    ) -> terrars::ListRef<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign_IamPolicyConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_policy_constraints", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteam_WorkerAccessConfigurationDynamic {
    s3_presign: std::option::Option<terrars::DynamicBlock<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkteam_WorkerAccessConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_presign: std::option::Option<std::vec::Vec<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign>>,
    dynamic: SagemakerWorkteam_WorkerAccessConfigurationDynamic,
}

impl SagemakerWorkteam_WorkerAccessConfiguration {
    #[doc = "Set the field `s3_presign`.\n"]
    pub fn set_s3_presign(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerWorkteam_WorkerAccessConfiguration_S3Presign>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_presign = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_presign = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerWorkteam_WorkerAccessConfiguration {
    type O = terrars::BlockAssignable<SagemakerWorkteam_WorkerAccessConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteam_WorkerAccessConfiguration {}

impl BuildSagemakerWorkteam_WorkerAccessConfiguration {
    pub fn build(self) -> SagemakerWorkteam_WorkerAccessConfiguration {
        SagemakerWorkteam_WorkerAccessConfiguration {
            s3_presign: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerWorkteam_WorkerAccessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerWorkteam_WorkerAccessConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerWorkteam_WorkerAccessConfigurationRef {
        SagemakerWorkteam_WorkerAccessConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteam_WorkerAccessConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_presign` after provisioning.\n"]
    pub fn s3_presign(&self) -> terrars::ListRef<SagemakerWorkteam_WorkerAccessConfiguration_S3PresignRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_presign", self.base))
    }
}
