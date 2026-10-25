#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectUserData_ {
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
    directory_user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchy_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    routing_profile_id: terrars::PrimField<std::string::String>,
    security_profile_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_info: std::option::Option<std::vec::Vec<ConnectUser_IdentityInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_config: std::option::Option<std::vec::Vec<ConnectUser_PhoneConfig>>,
    dynamic: ConnectUserDynamic,
}

struct ConnectUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectUserData_>,
}

#[derive(Clone)]
pub struct ConnectUser(std::rc::Rc<ConnectUser_>);

impl ConnectUser {
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

    #[doc = "Set the field `directory_user_id`.\n"]
    pub fn set_directory_user_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().directory_user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hierarchy_group_id`.\n"]
    pub fn set_hierarchy_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hierarchy_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `identity_info`.\n"]
    pub fn set_identity_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectUser_IdentityInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectUser_PhoneConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().phone_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.phone_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_user_id` after provisioning.\n"]
    pub fn directory_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hierarchy_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_profile_ids` after provisioning.\n"]
    pub fn security_profile_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_profile_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_info` after provisioning.\n"]
    pub fn identity_info(&self) -> terrars::ListRef<ConnectUser_IdentityInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> terrars::ListRef<ConnectUser_PhoneConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_config", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectUser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectUser { }

impl terrars::ToListMappable for ConnectUser {
    type O = terrars::ListRef<ConnectUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectUser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectUser {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub routing_profile_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_profile_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildConnectUser {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectUser {
        let out = ConnectUser(std::rc::Rc::new(ConnectUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectUserData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                directory_user_id: core::default::Default::default(),
                hierarchy_group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                password: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_profile_id: self.routing_profile_id,
                security_profile_ids: self.security_profile_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                identity_info: core::default::Default::default(),
                phone_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectUserRef {
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

    #[doc = "Get a reference to the value of field `directory_user_id` after provisioning.\n"]
    pub fn directory_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hierarchy_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_profile_ids` after provisioning.\n"]
    pub fn security_profile_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_profile_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_info` after provisioning.\n"]
    pub fn identity_info(&self) -> terrars::ListRef<ConnectUser_IdentityInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> terrars::ListRef<ConnectUser_PhoneConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ConnectUserDynamic {
    identity_info: std::option::Option<terrars::DynamicBlock<ConnectUser_IdentityInfo>>,
    phone_config: std::option::Option<terrars::DynamicBlock<ConnectUser_PhoneConfig>>,
}

#[derive(Serialize)]
pub struct ConnectUser_IdentityInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUser_IdentityInfo {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.first_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_email`.\n"]
    pub fn set_secondary_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectUser_IdentityInfo {
    type O = terrars::BlockAssignable<ConnectUser_IdentityInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUser_IdentityInfo {}

impl BuildConnectUser_IdentityInfo {
    pub fn build(self) -> ConnectUser_IdentityInfo {
        ConnectUser_IdentityInfo {
            email: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            secondary_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUser_IdentityInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUser_IdentityInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectUser_IdentityInfoRef {
        ConnectUser_IdentityInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUser_IdentityInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_email` after provisioning.\n"]
    pub fn secondary_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_email", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectUser_PhoneConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_contact_work_time_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_accept: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desk_phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_type: terrars::PrimField<std::string::String>,
}

impl ConnectUser_PhoneConfig {
    #[doc = "Set the field `after_contact_work_time_limit`.\n"]
    pub fn set_after_contact_work_time_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.after_contact_work_time_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_accept`.\n"]
    pub fn set_auto_accept(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_accept = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desk_phone_number`.\n"]
    pub fn set_desk_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.desk_phone_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectUser_PhoneConfig {
    type O = terrars::BlockAssignable<ConnectUser_PhoneConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUser_PhoneConfig {
    #[doc = ""]
    pub phone_type: terrars::PrimField<std::string::String>,
}

impl BuildConnectUser_PhoneConfig {
    pub fn build(self) -> ConnectUser_PhoneConfig {
        ConnectUser_PhoneConfig {
            after_contact_work_time_limit: core::default::Default::default(),
            auto_accept: core::default::Default::default(),
            desk_phone_number: core::default::Default::default(),
            phone_type: self.phone_type,
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUser_PhoneConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUser_PhoneConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectUser_PhoneConfigRef {
        ConnectUser_PhoneConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUser_PhoneConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_contact_work_time_limit` after provisioning.\n"]
    pub fn after_contact_work_time_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after_contact_work_time_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_accept", self.base))
    }

    #[doc = "Get a reference to the value of field `desk_phone_number` after provisioning.\n"]
    pub fn desk_phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desk_phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_type` after provisioning.\n"]
    pub fn phone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_type", self.base))
    }
}
