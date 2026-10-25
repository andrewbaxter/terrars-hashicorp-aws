#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontDistributionTenantData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudfrontDistributionTenant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfrontDistributionTenantData_>,
}

#[derive(Clone)]
pub struct DataCloudfrontDistributionTenant(std::rc::Rc<DataCloudfrontDistributionTenant_>);

impl DataCloudfrontDistributionTenant {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_group_id` after provisioning.\n"]
    pub fn connection_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customizations` after provisioning.\n"]
    pub fn customizations(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_CustomizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_DomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_certificate_request` after provisioning.\n"]
    pub fn managed_certificate_request(
        &self,
    ) -> terrars::ListRef<DataCloudfrontDistributionTenant_ManagedCertificateRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_certificate_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudfrontDistributionTenant {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfrontDistributionTenant { }

impl terrars::ToListMappable for DataCloudfrontDistributionTenant {
    type O = terrars::ListRef<DataCloudfrontDistributionTenantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfrontDistributionTenant_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudfront_distribution_tenant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontDistributionTenant {
    pub tf_id: std::string::String,
}

impl BuildDataCloudfrontDistributionTenant {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfrontDistributionTenant {
        let out = DataCloudfrontDistributionTenant(std::rc::Rc::new(DataCloudfrontDistributionTenant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfrontDistributionTenantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                domain: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontDistributionTenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontDistributionTenantRef {
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

    #[doc = "Get a reference to the value of field `connection_group_id` after provisioning.\n"]
    pub fn connection_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customizations` after provisioning.\n"]
    pub fn customizations(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_CustomizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_DomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_certificate_request` after provisioning.\n"]
    pub fn managed_certificate_request(
        &self,
    ) -> terrars::ListRef<DataCloudfrontDistributionTenant_ManagedCertificateRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_certificate_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_Customizations_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontDistributionTenant_Customizations_Certificate {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_Customizations_Certificate {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_Customizations_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_Customizations_Certificate {}

impl BuildDataCloudfrontDistributionTenant_Customizations_Certificate {
    pub fn build(self) -> DataCloudfrontDistributionTenant_Customizations_Certificate {
        DataCloudfrontDistributionTenant_Customizations_Certificate { arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_Customizations_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_Customizations_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontDistributionTenant_Customizations_CertificateRef {
        DataCloudfrontDistributionTenant_Customizations_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_Customizations_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_Customizations_GeoRestriction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restriction_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontDistributionTenant_Customizations_GeoRestriction {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restriction_type`.\n"]
    pub fn set_restriction_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restriction_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_Customizations_GeoRestriction {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_Customizations_GeoRestriction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_Customizations_GeoRestriction {}

impl BuildDataCloudfrontDistributionTenant_Customizations_GeoRestriction {
    pub fn build(self) -> DataCloudfrontDistributionTenant_Customizations_GeoRestriction {
        DataCloudfrontDistributionTenant_Customizations_GeoRestriction {
            locations: core::default::Default::default(),
            restriction_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
        DataCloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }

    #[doc = "Get a reference to the value of field `restriction_type` after provisioning.\n"]
    pub fn restriction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_Customizations_WebAcl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontDistributionTenant_Customizations_WebAcl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_Customizations_WebAcl {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_Customizations_WebAcl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_Customizations_WebAcl {}

impl BuildDataCloudfrontDistributionTenant_Customizations_WebAcl {
    pub fn build(self) -> DataCloudfrontDistributionTenant_Customizations_WebAcl {
        DataCloudfrontDistributionTenant_Customizations_WebAcl {
            action: core::default::Default::default(),
            arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_Customizations_WebAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_Customizations_WebAclRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontDistributionTenant_Customizations_WebAclRef {
        DataCloudfrontDistributionTenant_Customizations_WebAclRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_Customizations_WebAclRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_Customizations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::ListField<DataCloudfrontDistributionTenant_Customizations_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_restriction: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontDistributionTenant_Customizations_GeoRestriction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_acl: std::option::Option<terrars::ListField<DataCloudfrontDistributionTenant_Customizations_WebAcl>>,
}

impl DataCloudfrontDistributionTenant_Customizations {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontDistributionTenant_Customizations_Certificate>>,
    ) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo_restriction`.\n"]
    pub fn set_geo_restriction(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudfrontDistributionTenant_Customizations_GeoRestriction>>,
    ) -> Self {
        self.geo_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_acl`.\n"]
    pub fn set_web_acl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontDistributionTenant_Customizations_WebAcl>>,
    ) -> Self {
        self.web_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_Customizations {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_Customizations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_Customizations {}

impl BuildDataCloudfrontDistributionTenant_Customizations {
    pub fn build(self) -> DataCloudfrontDistributionTenant_Customizations {
        DataCloudfrontDistributionTenant_Customizations {
            certificate: core::default::Default::default(),
            geo_restriction: core::default::Default::default(),
            web_acl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_CustomizationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_CustomizationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontDistributionTenant_CustomizationsRef {
        DataCloudfrontDistributionTenant_CustomizationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_CustomizationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_Customizations_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_restriction` after provisioning.\n"]
    pub fn geo_restriction(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_Customizations_GeoRestrictionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `web_acl` after provisioning.\n"]
    pub fn web_acl(&self) -> terrars::ListRef<DataCloudfrontDistributionTenant_Customizations_WebAclRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_Domains {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontDistributionTenant_Domains {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_Domains {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_Domains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_Domains {}

impl BuildDataCloudfrontDistributionTenant_Domains {
    pub fn build(self) -> DataCloudfrontDistributionTenant_Domains {
        DataCloudfrontDistributionTenant_Domains {
            domain: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_DomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_DomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfrontDistributionTenant_DomainsRef {
        DataCloudfrontDistributionTenant_DomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_DomainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_ManagedCertificateRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_transparency_logging_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_token_host: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontDistributionTenant_ManagedCertificateRequest {
    #[doc = "Set the field `certificate_transparency_logging_preference`.\n"]
    pub fn set_certificate_transparency_logging_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_transparency_logging_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_domain_name`.\n"]
    pub fn set_primary_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.primary_domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation_token_host`.\n"]
    pub fn set_validation_token_host(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.validation_token_host = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_ManagedCertificateRequest {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_ManagedCertificateRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_ManagedCertificateRequest {}

impl BuildDataCloudfrontDistributionTenant_ManagedCertificateRequest {
    pub fn build(self) -> DataCloudfrontDistributionTenant_ManagedCertificateRequest {
        DataCloudfrontDistributionTenant_ManagedCertificateRequest {
            certificate_transparency_logging_preference: core::default::Default::default(),
            primary_domain_name: core::default::Default::default(),
            validation_token_host: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_ManagedCertificateRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_ManagedCertificateRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontDistributionTenant_ManagedCertificateRequestRef {
        DataCloudfrontDistributionTenant_ManagedCertificateRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_ManagedCertificateRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_transparency_logging_preference` after provisioning.\n"]
    pub fn certificate_transparency_logging_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_transparency_logging_preference", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `primary_domain_name` after provisioning.\n"]
    pub fn primary_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_token_host` after provisioning.\n"]
    pub fn validation_token_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_token_host", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontDistributionTenant_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontDistributionTenant_Parameters {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontDistributionTenant_Parameters {
    type O = terrars::BlockAssignable<DataCloudfrontDistributionTenant_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontDistributionTenant_Parameters {}

impl BuildDataCloudfrontDistributionTenant_Parameters {
    pub fn build(self) -> DataCloudfrontDistributionTenant_Parameters {
        DataCloudfrontDistributionTenant_Parameters {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontDistributionTenant_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontDistributionTenant_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontDistributionTenant_ParametersRef {
        DataCloudfrontDistributionTenant_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontDistributionTenant_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
