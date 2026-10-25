#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreUsersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    identity_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataIdentitystoreUsers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIdentitystoreUsersData_>,
}

#[derive(Clone)]
pub struct DataIdentitystoreUsers(std::rc::Rc<DataIdentitystoreUsers_>);

impl DataIdentitystoreUsers {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> terrars::ListRef<DataIdentitystoreUsers_UsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }
}

impl terrars::Referable for DataIdentitystoreUsers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIdentitystoreUsers { }

impl terrars::ToListMappable for DataIdentitystoreUsers {
    type O = terrars::ListRef<DataIdentitystoreUsersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIdentitystoreUsers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_identitystore_users".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreUsers {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_store_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreUsers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIdentitystoreUsers {
        let out = DataIdentitystoreUsers(std::rc::Rc::new(DataIdentitystoreUsers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIdentitystoreUsersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIdentitystoreUsersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> terrars::ListRef<DataIdentitystoreUsers_UsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreUsers_Users_Addresses {
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

impl DataIdentitystoreUsers_Users_Addresses {
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

impl terrars::ToListMappable for DataIdentitystoreUsers_Users_Addresses {
    type O = terrars::BlockAssignable<DataIdentitystoreUsers_Users_Addresses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUsers_Users_Addresses {}

impl BuildDataIdentitystoreUsers_Users_Addresses {
    pub fn build(self) -> DataIdentitystoreUsers_Users_Addresses {
        DataIdentitystoreUsers_Users_Addresses {
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

#[allow(dead_code)]
pub struct DataIdentitystoreUsers_Users_AddressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsers_Users_AddressesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUsers_Users_AddressesRef {
        DataIdentitystoreUsers_Users_AddressesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUsers_Users_AddressesRef {
    #[allow(dead_code)]
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
pub struct DataIdentitystoreUsers_Users_Emails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUsers_Users_Emails {
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

impl terrars::ToListMappable for DataIdentitystoreUsers_Users_Emails {
    type O = terrars::BlockAssignable<DataIdentitystoreUsers_Users_Emails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUsers_Users_Emails {}

impl BuildDataIdentitystoreUsers_Users_Emails {
    pub fn build(self) -> DataIdentitystoreUsers_Users_Emails {
        DataIdentitystoreUsers_Users_Emails {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreUsers_Users_EmailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsers_Users_EmailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUsers_Users_EmailsRef {
        DataIdentitystoreUsers_Users_EmailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUsers_Users_EmailsRef {
    #[allow(dead_code)]
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
pub struct DataIdentitystoreUsers_Users_ExternalIds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUsers_Users_ExternalIds {
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

impl terrars::ToListMappable for DataIdentitystoreUsers_Users_ExternalIds {
    type O = terrars::BlockAssignable<DataIdentitystoreUsers_Users_ExternalIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUsers_Users_ExternalIds {}

impl BuildDataIdentitystoreUsers_Users_ExternalIds {
    pub fn build(self) -> DataIdentitystoreUsers_Users_ExternalIds {
        DataIdentitystoreUsers_Users_ExternalIds {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreUsers_Users_ExternalIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsers_Users_ExternalIdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUsers_Users_ExternalIdsRef {
        DataIdentitystoreUsers_Users_ExternalIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUsers_Users_ExternalIdsRef {
    #[allow(dead_code)]
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
pub struct DataIdentitystoreUsers_Users_Name {
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

impl DataIdentitystoreUsers_Users_Name {
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

impl terrars::ToListMappable for DataIdentitystoreUsers_Users_Name {
    type O = terrars::BlockAssignable<DataIdentitystoreUsers_Users_Name>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUsers_Users_Name {}

impl BuildDataIdentitystoreUsers_Users_Name {
    pub fn build(self) -> DataIdentitystoreUsers_Users_Name {
        DataIdentitystoreUsers_Users_Name {
            family_name: core::default::Default::default(),
            formatted: core::default::Default::default(),
            given_name: core::default::Default::default(),
            honorific_prefix: core::default::Default::default(),
            honorific_suffix: core::default::Default::default(),
            middle_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreUsers_Users_NameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsers_Users_NameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUsers_Users_NameRef {
        DataIdentitystoreUsers_Users_NameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUsers_Users_NameRef {
    #[allow(dead_code)]
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
pub struct DataIdentitystoreUsers_Users_PhoneNumbers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUsers_Users_PhoneNumbers {
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

impl terrars::ToListMappable for DataIdentitystoreUsers_Users_PhoneNumbers {
    type O = terrars::BlockAssignable<DataIdentitystoreUsers_Users_PhoneNumbers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUsers_Users_PhoneNumbers {}

impl BuildDataIdentitystoreUsers_Users_PhoneNumbers {
    pub fn build(self) -> DataIdentitystoreUsers_Users_PhoneNumbers {
        DataIdentitystoreUsers_Users_PhoneNumbers {
            primary: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreUsers_Users_PhoneNumbersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsers_Users_PhoneNumbersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUsers_Users_PhoneNumbersRef {
        DataIdentitystoreUsers_Users_PhoneNumbersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUsers_Users_PhoneNumbersRef {
    #[allow(dead_code)]
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
pub struct DataIdentitystoreUsers_Users {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addresses: std::option::Option<terrars::ListField<DataIdentitystoreUsers_Users_Addresses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<DataIdentitystoreUsers_Users_Emails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ids: std::option::Option<terrars::ListField<DataIdentitystoreUsers_Users_ExternalIds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_store_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locale: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::ListField<DataIdentitystoreUsers_Users_Name>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nickname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_numbers: std::option::Option<terrars::ListField<DataIdentitystoreUsers_Users_PhoneNumbers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreUsers_Users {
    #[doc = "Set the field `addresses`.\n"]
    pub fn set_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIdentitystoreUsers_Users_Addresses>>,
    ) -> Self {
        self.addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIdentitystoreUsers_Users_Emails>>,
    ) -> Self {
        self.emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ids`.\n"]
    pub fn set_external_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIdentitystoreUsers_Users_ExternalIds>>,
    ) -> Self {
        self.external_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_store_id`.\n"]
    pub fn set_identity_store_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_store_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIdentitystoreUsers_Users_Name>>,
    ) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nickname`.\n"]
    pub fn set_nickname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nickname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_numbers`.\n"]
    pub fn set_phone_numbers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIdentitystoreUsers_Users_PhoneNumbers>>,
    ) -> Self {
        self.phone_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_language`.\n"]
    pub fn set_preferred_language(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preferred_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile_url`.\n"]
    pub fn set_profile_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_name`.\n"]
    pub fn set_user_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_status`.\n"]
    pub fn set_user_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_type`.\n"]
    pub fn set_user_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreUsers_Users {
    type O = terrars::BlockAssignable<DataIdentitystoreUsers_Users>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreUsers_Users {}

impl BuildDataIdentitystoreUsers_Users {
    pub fn build(self) -> DataIdentitystoreUsers_Users {
        DataIdentitystoreUsers_Users {
            addresses: core::default::Default::default(),
            display_name: core::default::Default::default(),
            emails: core::default::Default::default(),
            external_ids: core::default::Default::default(),
            identity_store_id: core::default::Default::default(),
            locale: core::default::Default::default(),
            name: core::default::Default::default(),
            nickname: core::default::Default::default(),
            phone_numbers: core::default::Default::default(),
            preferred_language: core::default::Default::default(),
            profile_url: core::default::Default::default(),
            timezone: core::default::Default::default(),
            title: core::default::Default::default(),
            user_id: core::default::Default::default(),
            user_name: core::default::Default::default(),
            user_status: core::default::Default::default(),
            user_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreUsers_UsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreUsers_UsersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreUsers_UsersRef {
        DataIdentitystoreUsers_UsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreUsers_UsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<DataIdentitystoreUsers_Users_AddressesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<DataIdentitystoreUsers_Users_EmailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<DataIdentitystoreUsers_Users_ExternalIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.base))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<DataIdentitystoreUsers_Users_NameRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `nickname` after provisioning.\n"]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_numbers` after provisioning.\n"]
    pub fn phone_numbers(&self) -> terrars::ListRef<DataIdentitystoreUsers_Users_PhoneNumbersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_numbers", self.base))
    }

    #[doc = "Get a reference to the value of field `preferred_language` after provisioning.\n"]
    pub fn preferred_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_language", self.base))
    }

    #[doc = "Get a reference to the value of field `profile_url` after provisioning.\n"]
    pub fn profile_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_url", self.base))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }

    #[doc = "Get a reference to the value of field `user_status` after provisioning.\n"]
    pub fn user_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_status", self.base))
    }

    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_type", self.base))
    }
}
