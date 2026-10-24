use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAcmpcaCertificateAuthorityData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataAcmpcaCertificateAuthority_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAcmpcaCertificateAuthorityData>,
}

#[derive(Clone)]
pub struct DataAcmpcaCertificateAuthority(std::rc::Rc<DataAcmpcaCertificateAuthority_>);

impl DataAcmpcaCertificateAuthority {
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(
        &self,
    ) -> terrars::ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_mode", self.extract_ref()))
    }
}

impl terrars::Referable for DataAcmpcaCertificateAuthority {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAcmpcaCertificateAuthority { }

impl terrars::ToListMappable for DataAcmpcaCertificateAuthority {
    type O = terrars::ListRef<DataAcmpcaCertificateAuthorityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAcmpcaCertificateAuthority_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_acmpca_certificate_authority".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAcmpcaCertificateAuthority {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataAcmpcaCertificateAuthority {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAcmpcaCertificateAuthority {
        let out = DataAcmpcaCertificateAuthority(std::rc::Rc::new(DataAcmpcaCertificateAuthority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAcmpcaCertificateAuthorityData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAcmpcaCertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAcmpcaCertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAcmpcaCertificateAuthorityRef {
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(
        &self,
    ) -> terrars::ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_mode", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_cname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_object_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    #[doc = "Set the field `custom_cname`.\n"]
    pub fn set_custom_cname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_cname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiration_in_days`.\n"]
    pub fn set_expiration_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.expiration_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_object_acl`.\n"]
    pub fn set_s3_object_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_object_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    type O = terrars::BlockAssignable<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {}

impl BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
    pub fn build(self) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl {
            custom_cname: core::default::Default::default(),
            enabled: core::default::Default::default(),
            expiration_in_days: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_object_acl: core::default::Default::default(),
        }
    }
}

pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_cname` after provisioning.\n"]
    pub fn custom_cname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_cname", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration_in_days` after provisioning.\n"]
    pub fn expiration_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_acl` after provisioning.\n"]
    pub fn s3_object_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_custom_cname: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocsp_custom_cname`.\n"]
    pub fn set_ocsp_custom_cname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocsp_custom_cname = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    type O = terrars::BlockAssignable<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {}

impl BuildDataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
    pub fn build(self) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl {
            enabled: core::default::Default::default(),
            ocsp_custom_cname: core::default::Default::default(),
        }
    }
}

pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_custom_cname` after provisioning.\n"]
    pub fn ocsp_custom_cname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsp_custom_cname", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_configuration: std
    ::option
    ::Option<terrars::ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_configuration: std
    ::option
    ::Option<terrars::ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>>,
}

impl DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    #[doc = "Set the field `crl_configuration`.\n"]
    pub fn set_crl_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationEl>,
                    >,
    ) -> Self {
        self.crl_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocsp_configuration`.\n"]
    pub fn set_ocsp_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationEl>,
                    >,
    ) -> Self {
        self.ocsp_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    type O = terrars::BlockAssignable<DataAcmpcaCertificateAuthorityRevocationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAcmpcaCertificateAuthorityRevocationConfigurationEl {}

impl BuildDataAcmpcaCertificateAuthorityRevocationConfigurationEl {
    pub fn build(self) -> DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
        DataAcmpcaCertificateAuthorityRevocationConfigurationEl {
            crl_configuration: core::default::Default::default(),
            ocsp_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
        DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAcmpcaCertificateAuthorityRevocationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crl_configuration` after provisioning.\n"]
    pub fn crl_configuration(
        &self,
    ) -> terrars::ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElCrlConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crl_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_configuration` after provisioning.\n"]
    pub fn ocsp_configuration(
        &self,
    ) -> terrars::ListRef<DataAcmpcaCertificateAuthorityRevocationConfigurationElOcspConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ocsp_configuration", self.base))
    }
}
