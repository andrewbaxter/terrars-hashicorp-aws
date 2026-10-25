#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSesv2EmailIdentityData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    email_identity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataSesv2EmailIdentity_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSesv2EmailIdentityData_>,
}

#[derive(Clone)]
pub struct DataSesv2EmailIdentity(std::rc::Rc<DataSesv2EmailIdentity_>);

impl DataSesv2EmailIdentity {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(&self) -> terrars::ListRef<DataSesv2EmailIdentity_DkimSigningAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dkim_signing_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_for_sending_status", self.extract_ref()))
    }
}

impl terrars::Referable for DataSesv2EmailIdentity {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSesv2EmailIdentity { }

impl terrars::ToListMappable for DataSesv2EmailIdentity {
    type O = terrars::ListRef<DataSesv2EmailIdentityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSesv2EmailIdentity_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_sesv2_email_identity".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesv2EmailIdentity {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub email_identity: terrars::PrimField<std::string::String>,
}

impl BuildDataSesv2EmailIdentity {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSesv2EmailIdentity {
        let out = DataSesv2EmailIdentity(std::rc::Rc::new(DataSesv2EmailIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSesv2EmailIdentityData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                email_identity: self.email_identity,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesv2EmailIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2EmailIdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSesv2EmailIdentityRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(&self) -> terrars::ListRef<DataSesv2EmailIdentity_DkimSigningAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dkim_signing_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_for_sending_status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSesv2EmailIdentity_DkimSigningAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_signing_key_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_signing_private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_signing_selector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_key_generation_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_signing_key_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_attributes_origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tokens: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataSesv2EmailIdentity_DkimSigningAttributes {
    #[doc = "Set the field `current_signing_key_length`.\n"]
    pub fn set_current_signing_key_length(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.current_signing_key_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_signing_private_key`.\n"]
    pub fn set_domain_signing_private_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.domain_signing_private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_signing_selector`.\n"]
    pub fn set_domain_signing_selector(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.domain_signing_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_key_generation_timestamp`.\n"]
    pub fn set_last_key_generation_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_key_generation_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_signing_key_length`.\n"]
    pub fn set_next_signing_key_length(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_signing_key_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signing_attributes_origin`.\n"]
    pub fn set_signing_attributes_origin(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.signing_attributes_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tokens`.\n"]
    pub fn set_tokens(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tokens = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2EmailIdentity_DkimSigningAttributes {
    type O = terrars::BlockAssignable<DataSesv2EmailIdentity_DkimSigningAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2EmailIdentity_DkimSigningAttributes {}

impl BuildDataSesv2EmailIdentity_DkimSigningAttributes {
    pub fn build(self) -> DataSesv2EmailIdentity_DkimSigningAttributes {
        DataSesv2EmailIdentity_DkimSigningAttributes {
            current_signing_key_length: core::default::Default::default(),
            domain_signing_private_key: core::default::Default::default(),
            domain_signing_selector: core::default::Default::default(),
            last_key_generation_timestamp: core::default::Default::default(),
            next_signing_key_length: core::default::Default::default(),
            signing_attributes_origin: core::default::Default::default(),
            status: core::default::Default::default(),
            tokens: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2EmailIdentity_DkimSigningAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2EmailIdentity_DkimSigningAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSesv2EmailIdentity_DkimSigningAttributesRef {
        DataSesv2EmailIdentity_DkimSigningAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2EmailIdentity_DkimSigningAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_signing_key_length` after provisioning.\n"]
    pub fn current_signing_key_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_signing_key_length", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_signing_private_key` after provisioning.\n"]
    pub fn domain_signing_private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_signing_private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_signing_selector` after provisioning.\n"]
    pub fn domain_signing_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_signing_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `last_key_generation_timestamp` after provisioning.\n"]
    pub fn last_key_generation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_key_generation_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `next_signing_key_length` after provisioning.\n"]
    pub fn next_signing_key_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_signing_key_length", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_attributes_origin` after provisioning.\n"]
    pub fn signing_attributes_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_attributes_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tokens", self.base))
    }
}
