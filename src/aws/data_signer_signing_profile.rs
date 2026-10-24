use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSignerSigningProfileData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataSignerSigningProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSignerSigningProfileData>,
}

#[derive(Clone)]
pub struct DataSignerSigningProfile(std::rc::Rc<DataSignerSigningProfile_>);

impl DataSignerSigningProfile {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> terrars::ListRef<DataSignerSigningProfileRevocationRecordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_validity_period` after provisioning.\n"]
    pub fn signature_validity_period(&self) -> terrars::ListRef<DataSignerSigningProfileSignatureValidityPeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signature_validity_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_arn` after provisioning.\n"]
    pub fn version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_arn", self.extract_ref()))
    }
}

impl terrars::Referable for DataSignerSigningProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSignerSigningProfile { }

impl terrars::ToListMappable for DataSignerSigningProfile {
    type O = terrars::ListRef<DataSignerSigningProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSignerSigningProfile_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_signer_signing_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSignerSigningProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataSignerSigningProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSignerSigningProfile {
        let out = DataSignerSigningProfile(std::rc::Rc::new(DataSignerSigningProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSignerSigningProfileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSignerSigningProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSignerSigningProfileRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> terrars::ListRef<DataSignerSigningProfileRevocationRecordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_validity_period` after provisioning.\n"]
    pub fn signature_validity_period(&self) -> terrars::ListRef<DataSignerSigningProfileSignatureValidityPeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signature_validity_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_arn` after provisioning.\n"]
    pub fn version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_arn", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningProfileRevocationRecordEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revocation_effective_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSignerSigningProfileRevocationRecordEl {
    #[doc = "Set the field `revocation_effective_from`.\n"]
    pub fn set_revocation_effective_from(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.revocation_effective_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_at`.\n"]
    pub fn set_revoked_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revoked_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_by`.\n"]
    pub fn set_revoked_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revoked_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningProfileRevocationRecordEl {
    type O = terrars::BlockAssignable<DataSignerSigningProfileRevocationRecordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningProfileRevocationRecordEl {}

impl BuildDataSignerSigningProfileRevocationRecordEl {
    pub fn build(self) -> DataSignerSigningProfileRevocationRecordEl {
        DataSignerSigningProfileRevocationRecordEl {
            revocation_effective_from: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningProfileRevocationRecordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningProfileRevocationRecordElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSignerSigningProfileRevocationRecordElRef {
        DataSignerSigningProfileRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningProfileRevocationRecordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revocation_effective_from` after provisioning.\n"]
    pub fn revocation_effective_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revocation_effective_from", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_at` after provisioning.\n"]
    pub fn revoked_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoked_at", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_by` after provisioning.\n"]
    pub fn revoked_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoked_by", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningProfileSignatureValidityPeriodEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSignerSigningProfileSignatureValidityPeriodEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningProfileSignatureValidityPeriodEl {
    type O = terrars::BlockAssignable<DataSignerSigningProfileSignatureValidityPeriodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningProfileSignatureValidityPeriodEl {}

impl BuildDataSignerSigningProfileSignatureValidityPeriodEl {
    pub fn build(self) -> DataSignerSigningProfileSignatureValidityPeriodEl {
        DataSignerSigningProfileSignatureValidityPeriodEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningProfileSignatureValidityPeriodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningProfileSignatureValidityPeriodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSignerSigningProfileSignatureValidityPeriodElRef {
        DataSignerSigningProfileSignatureValidityPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningProfileSignatureValidityPeriodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
