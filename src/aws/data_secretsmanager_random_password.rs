#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecretsmanagerRandomPasswordData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_characters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_lowercase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_numbers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_punctuation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_uppercase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_space: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_each_included_type: std::option::Option<terrars::PrimField<bool>>,
}

struct DataSecretsmanagerRandomPassword_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecretsmanagerRandomPasswordData_>,
}

#[derive(Clone)]
pub struct DataSecretsmanagerRandomPassword(std::rc::Rc<DataSecretsmanagerRandomPassword_>);

impl DataSecretsmanagerRandomPassword {
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

    #[doc = "Set the field `exclude_characters`.\n"]
    pub fn set_exclude_characters(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().exclude_characters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_lowercase`.\n"]
    pub fn set_exclude_lowercase(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_lowercase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_numbers`.\n"]
    pub fn set_exclude_numbers(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_punctuation`.\n"]
    pub fn set_exclude_punctuation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_punctuation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_uppercase`.\n"]
    pub fn set_exclude_uppercase(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_uppercase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_space`.\n"]
    pub fn set_include_space(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_space = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_length`.\n"]
    pub fn set_password_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_each_included_type`.\n"]
    pub fn set_require_each_included_type(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_each_included_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `exclude_characters` after provisioning.\n"]
    pub fn exclude_characters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_lowercase` after provisioning.\n"]
    pub fn exclude_lowercase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_lowercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_numbers` after provisioning.\n"]
    pub fn exclude_numbers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_punctuation` after provisioning.\n"]
    pub fn exclude_punctuation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_punctuation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_uppercase` after provisioning.\n"]
    pub fn exclude_uppercase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_uppercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_space` after provisioning.\n"]
    pub fn include_space(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_space", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_length` after provisioning.\n"]
    pub fn password_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `random_password` after provisioning.\n"]
    pub fn random_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.random_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_each_included_type` after provisioning.\n"]
    pub fn require_each_included_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_each_included_type", self.extract_ref()))
    }
}

impl terrars::Referable for DataSecretsmanagerRandomPassword {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecretsmanagerRandomPassword { }

impl terrars::ToListMappable for DataSecretsmanagerRandomPassword {
    type O = terrars::ListRef<DataSecretsmanagerRandomPasswordRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecretsmanagerRandomPassword_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_secretsmanager_random_password".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretsmanagerRandomPassword {
    pub tf_id: std::string::String,
}

impl BuildDataSecretsmanagerRandomPassword {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecretsmanagerRandomPassword {
        let out = DataSecretsmanagerRandomPassword(std::rc::Rc::new(DataSecretsmanagerRandomPassword_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSecretsmanagerRandomPasswordData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                exclude_characters: core::default::Default::default(),
                exclude_lowercase: core::default::Default::default(),
                exclude_numbers: core::default::Default::default(),
                exclude_punctuation: core::default::Default::default(),
                exclude_uppercase: core::default::Default::default(),
                id: core::default::Default::default(),
                include_space: core::default::Default::default(),
                password_length: core::default::Default::default(),
                region: core::default::Default::default(),
                require_each_included_type: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretsmanagerRandomPasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretsmanagerRandomPasswordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecretsmanagerRandomPasswordRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `exclude_characters` after provisioning.\n"]
    pub fn exclude_characters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_characters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_lowercase` after provisioning.\n"]
    pub fn exclude_lowercase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_lowercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_numbers` after provisioning.\n"]
    pub fn exclude_numbers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_numbers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_punctuation` after provisioning.\n"]
    pub fn exclude_punctuation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_punctuation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_uppercase` after provisioning.\n"]
    pub fn exclude_uppercase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_uppercase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_space` after provisioning.\n"]
    pub fn include_space(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_space", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_length` after provisioning.\n"]
    pub fn password_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `random_password` after provisioning.\n"]
    pub fn random_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.random_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_each_included_type` after provisioning.\n"]
    pub fn require_each_included_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_each_included_type", self.extract_ref()))
    }
}
