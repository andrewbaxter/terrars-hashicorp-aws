use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreUserData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    identity_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternate_identifier: std::option::Option<std::vec::Vec<DataIdentitystoreUserAlternateIdentifierEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataIdentitystoreUserFilterEl>>,
    dynamic: DataIdentitystoreUserDynamic,
}

struct DataIdentitystoreUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIdentitystoreUserData>,
}

#[derive(Clone)]
pub struct DataIdentitystoreUser(std::rc::Rc<DataIdentitystoreUser_>);

impl DataIdentitystoreUser {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alternate_identifier`.\n"]
    pub fn set_alternate_identifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIdentitystoreUserAlternateIdentifierEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alternate_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alternate_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIdentitystoreUserFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<DataIdentitystoreUserAddressesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<DataIdentitystoreUserEmailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<DataIdentitystoreUserExternalIdsElRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<DataIdentitystoreUserNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> terrars::ListRef<DataIdentitystoreUserPhoneNumbersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> terrars::ListRef<DataIdentitystoreUserAlternateIdentifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataIdentitystoreUserFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl terrars::Referable for DataIdentitystoreUser {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIdentitystoreUser { }

impl terrars::ToListMappable for DataIdentitystoreUser {
    type O = terrars::ListRef<DataIdentitystoreUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIdentitystoreUser_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_identitystore_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreUser {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_store_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreUser {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIdentitystoreUser {
        let out = DataIdentitystoreUser(std::rc::Rc::new(DataIdentitystoreUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIdentitystoreUserData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                identity_store_id: self.identity_store_id,
                user_id: core::default::Default::default(),
                alternate_identifier: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIdentitystoreUserRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<DataIdentitystoreUserAddressesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<DataIdentitystoreUserEmailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<DataIdentitystoreUserExternalIdsElRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<DataIdentitystoreUserNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> terrars::ListRef<DataIdentitystoreUserPhoneNumbersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> terrars::ListRef<DataIdentitystoreUserAlternateIdentifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataIdentitystoreUserFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreUserAddressesEl {
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

impl DataIdentitystoreUserAddressesEl {
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

impl terrars::ToListMappable for DataIdentitystoreUserAddressesEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserAddressesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAddressesEl {}

impl BuildDataIdentitystoreUserAddressesEl {
    pub fn build(self) -> DataIdentitystoreUserAddressesEl {
        DataIdentitystoreUserAddressesEl {
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

pub struct DataIdentitystoreUserAddressesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserAddressesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserAddressesElRef {
        DataIdentitystoreUserAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAddressesElRef {
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
pub struct DataIdentitystoreUserEmailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUserEmailsEl {
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

impl terrars::ToListMappable for DataIdentitystoreUserEmailsEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserEmailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserEmailsEl {}

impl BuildDataIdentitystoreUserEmailsEl {
    pub fn build(self) -> DataIdentitystoreUserEmailsEl {
        DataIdentitystoreUserEmailsEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserEmailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserEmailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserEmailsElRef {
        DataIdentitystoreUserEmailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserEmailsElRef {
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
pub struct DataIdentitystoreUserExternalIdsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUserExternalIdsEl {
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

impl terrars::ToListMappable for DataIdentitystoreUserExternalIdsEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserExternalIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserExternalIdsEl {}

impl BuildDataIdentitystoreUserExternalIdsEl {
    pub fn build(self) -> DataIdentitystoreUserExternalIdsEl {
        DataIdentitystoreUserExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserExternalIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserExternalIdsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserExternalIdsElRef {
        DataIdentitystoreUserExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserExternalIdsElRef {
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
pub struct DataIdentitystoreUserNameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    formatted: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    given_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honorific_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honorific_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    middle_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUserNameEl {
    #[doc = "Set the field `family_name`.\n"]
    pub fn set_family_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.family_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `formatted`.\n"]
    pub fn set_formatted(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.formatted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `given_name`.\n"]
    pub fn set_given_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.given_name = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataIdentitystoreUserNameEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserNameEl {}

impl BuildDataIdentitystoreUserNameEl {
    pub fn build(self) -> DataIdentitystoreUserNameEl {
        DataIdentitystoreUserNameEl {
            family_name: core::default::Default::default(),
            formatted: core::default::Default::default(),
            given_name: core::default::Default::default(),
            honorific_prefix: core::default::Default::default(),
            honorific_suffix: core::default::Default::default(),
            middle_name: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserNameElRef {
        DataIdentitystoreUserNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserNameElRef {
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
pub struct DataIdentitystoreUserPhoneNumbersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUserPhoneNumbersEl {
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

impl terrars::ToListMappable for DataIdentitystoreUserPhoneNumbersEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserPhoneNumbersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserPhoneNumbersEl {}

impl BuildDataIdentitystoreUserPhoneNumbersEl {
    pub fn build(self) -> DataIdentitystoreUserPhoneNumbersEl {
        DataIdentitystoreUserPhoneNumbersEl {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserPhoneNumbersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserPhoneNumbersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserPhoneNumbersElRef {
        DataIdentitystoreUserPhoneNumbersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserPhoneNumbersElRef {
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
pub struct DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    id: terrars::PrimField<std::string::String>,
    issuer: terrars::PrimField<std::string::String>,
}

impl DataIdentitystoreUserAlternateIdentifierElExternalIdEl { }

impl terrars::ToListMappable for DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreUserAlternateIdentifierElExternalIdEl {
    pub fn build(self) -> DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
        DataIdentitystoreUserAlternateIdentifierElExternalIdEl {
            id: self.id,
            issuer: self.issuer,
        }
    }
}

pub struct DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
        DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAlternateIdentifierElExternalIdElRef {
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
pub struct DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    attribute_path: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
}

impl DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl { }

impl terrars::ToListMappable for DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    #[doc = ""]
    pub attribute_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub attribute_value: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
    pub fn build(self) -> DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
        DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl {
            attribute_path: self.attribute_path,
            attribute_value: self.attribute_value,
        }
    }
}

pub struct DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
        DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_path` after provisioning.\n"]
    pub fn attribute_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_path", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreUserAlternateIdentifierElDynamic {
    external_id: std::option::Option<terrars::DynamicBlock<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>>,
    unique_attribute: std
    ::option
    ::Option<terrars::DynamicBlock<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>>,
}

#[derive(Serialize)]
pub struct DataIdentitystoreUserAlternateIdentifierEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<std::vec::Vec<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_attribute: std::option::Option<std::vec::Vec<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>>,
    dynamic: DataIdentitystoreUserAlternateIdentifierElDynamic,
}

impl DataIdentitystoreUserAlternateIdentifierEl {
    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIdentitystoreUserAlternateIdentifierElExternalIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unique_attribute`.\n"]
    pub fn set_unique_attribute(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unique_attribute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unique_attribute = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreUserAlternateIdentifierEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserAlternateIdentifierEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserAlternateIdentifierEl {}

impl BuildDataIdentitystoreUserAlternateIdentifierEl {
    pub fn build(self) -> DataIdentitystoreUserAlternateIdentifierEl {
        DataIdentitystoreUserAlternateIdentifierEl {
            external_id: core::default::Default::default(),
            unique_attribute: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreUserAlternateIdentifierElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserAlternateIdentifierElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserAlternateIdentifierElRef {
        DataIdentitystoreUserAlternateIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserAlternateIdentifierElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::ListRef<DataIdentitystoreUserAlternateIdentifierElExternalIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_attribute` after provisioning.\n"]
    pub fn unique_attribute(&self) -> terrars::ListRef<DataIdentitystoreUserAlternateIdentifierElUniqueAttributeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unique_attribute", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreUserFilterEl {
    attribute_path: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
}

impl DataIdentitystoreUserFilterEl { }

impl terrars::ToListMappable for DataIdentitystoreUserFilterEl {
    type O = terrars::BlockAssignable<DataIdentitystoreUserFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUserFilterEl {
    #[doc = ""]
    pub attribute_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub attribute_value: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreUserFilterEl {
    pub fn build(self) -> DataIdentitystoreUserFilterEl {
        DataIdentitystoreUserFilterEl {
            attribute_path: self.attribute_path,
            attribute_value: self.attribute_value,
        }
    }
}

pub struct DataIdentitystoreUserFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUserFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUserFilterElRef {
        DataIdentitystoreUserFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUserFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_path` after provisioning.\n"]
    pub fn attribute_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_path", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreUserDynamic {
    alternate_identifier: std::option::Option<terrars::DynamicBlock<DataIdentitystoreUserAlternateIdentifierEl>>,
    filter: std::option::Option<terrars::DynamicBlock<DataIdentitystoreUserFilterEl>>,
}
