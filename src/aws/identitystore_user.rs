use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IdentitystoreUserData {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    identity_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locale: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nickname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    user_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addresses: std::option::Option<std::vec::Vec<IdentitystoreUserAddressesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<std::vec::Vec<IdentitystoreUserEmailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<std::vec::Vec<IdentitystoreUserNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_numbers: std::option::Option<std::vec::Vec<IdentitystoreUserPhoneNumbersEl>>,
    dynamic: IdentitystoreUserDynamic,
}

struct IdentitystoreUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IdentitystoreUserData>,
}

#[derive(Clone)]
pub struct IdentitystoreUser(std::rc::Rc<IdentitystoreUser_>);

impl IdentitystoreUser {
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

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().locale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nickname`.\n"]
    pub fn set_nickname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nickname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_language`.\n"]
    pub fn set_preferred_language(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().preferred_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile_url`.\n"]
    pub fn set_profile_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().profile_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_type`.\n"]
    pub fn set_user_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `addresses`.\n"]
    pub fn set_addresses(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentitystoreUserAddressesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(self, v: impl std::convert::Into<terrars::BlockAssignable<IdentitystoreUserEmailsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().emails = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.emails = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::BlockAssignable<IdentitystoreUserNameEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_numbers`.\n"]
    pub fn set_phone_numbers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentitystoreUserPhoneNumbersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().phone_numbers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.phone_numbers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<IdentitystoreUserExternalIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_language` after provisioning.\n"]
    pub fn preferred_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_url` after provisioning.\n"]
    pub fn profile_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<IdentitystoreUserAddressesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<IdentitystoreUserEmailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<IdentitystoreUserNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> terrars::ListRef<IdentitystoreUserPhoneNumbersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
    }
}

impl terrars::Referable for IdentitystoreUser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IdentitystoreUser { }

impl terrars::ToListMappable for IdentitystoreUser {
    type O = terrars::ListRef<IdentitystoreUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IdentitystoreUser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_identitystore_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentitystoreUser {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub identity_store_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_name: terrars::PrimField<std::string::String>,
}

impl BuildIdentitystoreUser {
    pub fn build(self, stack: &mut terrars::Stack) -> IdentitystoreUser {
        let out = IdentitystoreUser(std::rc::Rc::new(IdentitystoreUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IdentitystoreUserData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                identity_store_id: self.identity_store_id,
                locale: core::default::Default::default(),
                nickname: core::default::Default::default(),
                preferred_language: core::default::Default::default(),
                profile_url: core::default::Default::default(),
                timezone: core::default::Default::default(),
                title: core::default::Default::default(),
                user_name: self.user_name,
                user_type: core::default::Default::default(),
                addresses: core::default::Default::default(),
                emails: core::default::Default::default(),
                name: core::default::Default::default(),
                phone_numbers: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentitystoreUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentitystoreUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentitystoreUserRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<IdentitystoreUserExternalIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_language` after provisioning.\n"]
    pub fn preferred_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_url` after provisioning.\n"]
    pub fn profile_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<IdentitystoreUserAddressesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<IdentitystoreUserEmailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<IdentitystoreUserNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> terrars::ListRef<IdentitystoreUserPhoneNumbersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserExternalIdsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentitystoreUserExternalIdsEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentitystoreUserExternalIdsEl {
    type O = terrars::BlockAssignable<IdentitystoreUserExternalIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserExternalIdsEl {}

impl BuildIdentitystoreUserExternalIdsEl {
    pub fn build(self) -> IdentitystoreUserExternalIdsEl {
        IdentitystoreUserExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserExternalIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentitystoreUserExternalIdsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentitystoreUserExternalIdsElRef {
        IdentitystoreUserExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserExternalIdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserAddressesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    formatted: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    street_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentitystoreUserAddressesEl {
    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `formatted`.\n"]
    pub fn set_formatted(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.formatted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\n"]
    pub fn set_locality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `street_address`.\n"]
    pub fn set_street_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.street_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentitystoreUserAddressesEl {
    type O = terrars::BlockAssignable<IdentitystoreUserAddressesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserAddressesEl {}

impl BuildIdentitystoreUserAddressesEl {
    pub fn build(self) -> IdentitystoreUserAddressesEl {
        IdentitystoreUserAddressesEl {
            country: core::default::Default::default(),
            formatted: core::default::Default::default(),
            locality: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            primary: core::default::Default::default(),
            region: core::default::Default::default(),
            street_address: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserAddressesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentitystoreUserAddressesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentitystoreUserAddressesElRef {
        IdentitystoreUserAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserAddressesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `formatted` after provisioning.\n"]
    pub fn formatted(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.formatted", self.base))
    }

    #[doc = "Get a reference to the value of field `locality` after provisioning.\n"]
    pub fn locality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `street_address` after provisioning.\n"]
    pub fn street_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.street_address", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserEmailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentitystoreUserEmailsEl {
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentitystoreUserEmailsEl {
    type O = terrars::BlockAssignable<IdentitystoreUserEmailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserEmailsEl {}

impl BuildIdentitystoreUserEmailsEl {
    pub fn build(self) -> IdentitystoreUserEmailsEl {
        IdentitystoreUserEmailsEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserEmailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentitystoreUserEmailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentitystoreUserEmailsElRef {
        IdentitystoreUserEmailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserEmailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserNameEl {
    family_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    formatted: std::option::Option<terrars::PrimField<std::string::String>>,
    given_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honorific_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honorific_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    middle_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentitystoreUserNameEl {
    #[doc = "Set the field `formatted`.\n"]
    pub fn set_formatted(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.formatted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `honorific_prefix`.\n"]
    pub fn set_honorific_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.honorific_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `honorific_suffix`.\n"]
    pub fn set_honorific_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.honorific_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `middle_name`.\n"]
    pub fn set_middle_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.middle_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentitystoreUserNameEl {
    type O = terrars::BlockAssignable<IdentitystoreUserNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserNameEl {
    #[doc = ""]
    pub family_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub given_name: terrars::PrimField<std::string::String>,
}

impl BuildIdentitystoreUserNameEl {
    pub fn build(self) -> IdentitystoreUserNameEl {
        IdentitystoreUserNameEl {
            family_name: self.family_name,
            formatted: core::default::Default::default(),
            given_name: self.given_name,
            honorific_prefix: core::default::Default::default(),
            honorific_suffix: core::default::Default::default(),
            middle_name: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentitystoreUserNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentitystoreUserNameElRef {
        IdentitystoreUserNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `family_name` after provisioning.\n"]
    pub fn family_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family_name", self.base))
    }

    #[doc = "Get a reference to the value of field `formatted` after provisioning.\n"]
    pub fn formatted(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.formatted", self.base))
    }

    #[doc = "Get a reference to the value of field `given_name` after provisioning.\n"]
    pub fn given_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.given_name", self.base))
    }

    #[doc = "Get a reference to the value of field `honorific_prefix` after provisioning.\n"]
    pub fn honorific_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.honorific_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `honorific_suffix` after provisioning.\n"]
    pub fn honorific_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.honorific_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `middle_name` after provisioning.\n"]
    pub fn middle_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.middle_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentitystoreUserPhoneNumbersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentitystoreUserPhoneNumbersEl {
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentitystoreUserPhoneNumbersEl {
    type O = terrars::BlockAssignable<IdentitystoreUserPhoneNumbersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentitystoreUserPhoneNumbersEl {}

impl BuildIdentitystoreUserPhoneNumbersEl {
    pub fn build(self) -> IdentitystoreUserPhoneNumbersEl {
        IdentitystoreUserPhoneNumbersEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct IdentitystoreUserPhoneNumbersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentitystoreUserPhoneNumbersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentitystoreUserPhoneNumbersElRef {
        IdentitystoreUserPhoneNumbersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentitystoreUserPhoneNumbersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentitystoreUserDynamic {
    addresses: std::option::Option<terrars::DynamicBlock<IdentitystoreUserAddressesEl>>,
    emails: std::option::Option<terrars::DynamicBlock<IdentitystoreUserEmailsEl>>,
    name: std::option::Option<terrars::DynamicBlock<IdentitystoreUserNameEl>>,
    phone_numbers: std::option::Option<terrars::DynamicBlock<IdentitystoreUserPhoneNumbersEl>>,
}
