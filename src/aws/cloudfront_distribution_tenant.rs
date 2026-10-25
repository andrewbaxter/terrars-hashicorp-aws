#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontDistributionTenantData_ {
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
    connection_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    distribution_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_deployment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customizations: std::option::Option<std::vec::Vec<CloudfrontDistributionTenant_Customizations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<std::vec::Vec<CloudfrontDistributionTenant_Domain>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_certificate_request: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistributionTenant_ManagedCertificateRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<CloudfrontDistributionTenant_Parameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudfrontDistributionTenant_Timeouts>,
    dynamic: CloudfrontDistributionTenantDynamic,
}

struct CloudfrontDistributionTenant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontDistributionTenantData_>,
}

#[derive(Clone)]
pub struct CloudfrontDistributionTenant(std::rc::Rc<CloudfrontDistributionTenant_>);

impl CloudfrontDistributionTenant {
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

    #[doc = "Set the field `connection_group_id`.\n"]
    pub fn set_connection_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connection_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `wait_for_deployment`.\n"]
    pub fn set_wait_for_deployment(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_deployment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customizations`.\n"]
    pub fn set_customizations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customizations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customizations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_Domain>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_certificate_request`.\n"]
    pub fn set_managed_certificate_request(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_ManagedCertificateRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_certificate_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_certificate_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_Parameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudfrontDistributionTenant_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_deployment` after provisioning.\n"]
    pub fn wait_for_deployment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customizations` after provisioning.\n"]
    pub fn customizations(&self) -> terrars::ListRef<CloudfrontDistributionTenant_CustomizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_certificate_request` after provisioning.\n"]
    pub fn managed_certificate_request(
        &self,
    ) -> terrars::ListRef<CloudfrontDistributionTenant_ManagedCertificateRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_certificate_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfrontDistributionTenant_TimeoutsRef {
        CloudfrontDistributionTenant_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudfrontDistributionTenant {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontDistributionTenant { }

impl terrars::ToListMappable for CloudfrontDistributionTenant {
    type O = terrars::ListRef<CloudfrontDistributionTenantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontDistributionTenant_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_distribution_tenant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontDistributionTenant {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub distribution_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionTenant {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontDistributionTenant {
        let out = CloudfrontDistributionTenant(std::rc::Rc::new(CloudfrontDistributionTenant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontDistributionTenantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_group_id: core::default::Default::default(),
                distribution_id: self.distribution_id,
                enabled: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                wait_for_deployment: core::default::Default::default(),
                customizations: core::default::Default::default(),
                domain: core::default::Default::default(),
                managed_certificate_request: core::default::Default::default(),
                parameter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontDistributionTenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontDistributionTenantRef {
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

    #[doc = "Get a reference to the value of field `connection_group_id` after provisioning.\n"]
    pub fn connection_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_deployment` after provisioning.\n"]
    pub fn wait_for_deployment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customizations` after provisioning.\n"]
    pub fn customizations(&self) -> terrars::ListRef<CloudfrontDistributionTenant_CustomizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_certificate_request` after provisioning.\n"]
    pub fn managed_certificate_request(
        &self,
    ) -> terrars::ListRef<CloudfrontDistributionTenant_ManagedCertificateRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_certificate_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfrontDistributionTenant_TimeoutsRef {
        CloudfrontDistributionTenant_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistributionTenantDynamic {
    customizations: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_Customizations>>,
    domain: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_Domain>>,
    managed_certificate_request: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_ManagedCertificateRequest>>,
    parameter: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_Parameter>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionTenant_Customizations_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionTenant_Customizations_Certificate {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionTenant_Customizations_Certificate {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Customizations_Certificate {}

impl BuildCloudfrontDistributionTenant_Customizations_Certificate {
    pub fn build(self) -> CloudfrontDistributionTenant_Customizations_Certificate {
        CloudfrontDistributionTenant_Customizations_Certificate { arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_Customizations_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_Customizations_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTenant_Customizations_CertificateRef {
        CloudfrontDistributionTenant_Customizations_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_Customizations_CertificateRef {
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
pub struct CloudfrontDistributionTenant_Customizations_GeoRestriction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restriction_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionTenant_Customizations_GeoRestriction {
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

impl terrars::ToListMappable for CloudfrontDistributionTenant_Customizations_GeoRestriction {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations_GeoRestriction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Customizations_GeoRestriction {}

impl BuildCloudfrontDistributionTenant_Customizations_GeoRestriction {
    pub fn build(self) -> CloudfrontDistributionTenant_Customizations_GeoRestriction {
        CloudfrontDistributionTenant_Customizations_GeoRestriction {
            locations: core::default::Default::default(),
            restriction_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
        CloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_Customizations_GeoRestrictionRef {
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
pub struct CloudfrontDistributionTenant_Customizations_WebAcl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionTenant_Customizations_WebAcl {
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

impl terrars::ToListMappable for CloudfrontDistributionTenant_Customizations_WebAcl {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations_WebAcl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Customizations_WebAcl {}

impl BuildCloudfrontDistributionTenant_Customizations_WebAcl {
    pub fn build(self) -> CloudfrontDistributionTenant_Customizations_WebAcl {
        CloudfrontDistributionTenant_Customizations_WebAcl {
            action: core::default::Default::default(),
            arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_Customizations_WebAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_Customizations_WebAclRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTenant_Customizations_WebAclRef {
        CloudfrontDistributionTenant_Customizations_WebAclRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_Customizations_WebAclRef {
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

#[derive(Serialize, Default)]
struct CloudfrontDistributionTenant_CustomizationsDynamic {
    certificate: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_Customizations_Certificate>>,
    geo_restriction: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_Customizations_GeoRestriction>>,
    web_acl: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionTenant_Customizations_WebAcl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionTenant_Customizations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<CloudfrontDistributionTenant_Customizations_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_restriction: std::option::Option<std::vec::Vec<CloudfrontDistributionTenant_Customizations_GeoRestriction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_acl: std::option::Option<std::vec::Vec<CloudfrontDistributionTenant_Customizations_WebAcl>>,
    dynamic: CloudfrontDistributionTenant_CustomizationsDynamic,
}

impl CloudfrontDistributionTenant_Customizations {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations_Certificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geo_restriction`.\n"]
    pub fn set_geo_restriction(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations_GeoRestriction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_restriction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_restriction = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_acl`.\n"]
    pub fn set_web_acl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations_WebAcl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_acl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_acl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionTenant_Customizations {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Customizations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Customizations {}

impl BuildCloudfrontDistributionTenant_Customizations {
    pub fn build(self) -> CloudfrontDistributionTenant_Customizations {
        CloudfrontDistributionTenant_Customizations {
            certificate: core::default::Default::default(),
            geo_restriction: core::default::Default::default(),
            web_acl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_CustomizationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_CustomizationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTenant_CustomizationsRef {
        CloudfrontDistributionTenant_CustomizationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_CustomizationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<CloudfrontDistributionTenant_Customizations_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_restriction` after provisioning.\n"]
    pub fn geo_restriction(&self) -> terrars::ListRef<CloudfrontDistributionTenant_Customizations_GeoRestrictionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `web_acl` after provisioning.\n"]
    pub fn web_acl(&self) -> terrars::ListRef<CloudfrontDistributionTenant_Customizations_WebAclRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionTenant_Domain {
    domain: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionTenant_Domain { }

impl terrars::ToListMappable for CloudfrontDistributionTenant_Domain {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Domain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Domain {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionTenant_Domain {
    pub fn build(self) -> CloudfrontDistributionTenant_Domain {
        CloudfrontDistributionTenant_Domain { domain: self.domain }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_DomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_DomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionTenant_DomainRef {
        CloudfrontDistributionTenant_DomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_DomainRef {
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
pub struct CloudfrontDistributionTenant_ManagedCertificateRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_transparency_logging_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_token_host: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionTenant_ManagedCertificateRequest {
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

impl terrars::ToListMappable for CloudfrontDistributionTenant_ManagedCertificateRequest {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_ManagedCertificateRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_ManagedCertificateRequest {}

impl BuildCloudfrontDistributionTenant_ManagedCertificateRequest {
    pub fn build(self) -> CloudfrontDistributionTenant_ManagedCertificateRequest {
        CloudfrontDistributionTenant_ManagedCertificateRequest {
            certificate_transparency_logging_preference: core::default::Default::default(),
            primary_domain_name: core::default::Default::default(),
            validation_token_host: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_ManagedCertificateRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_ManagedCertificateRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTenant_ManagedCertificateRequestRef {
        CloudfrontDistributionTenant_ManagedCertificateRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_ManagedCertificateRequestRef {
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
pub struct CloudfrontDistributionTenant_Parameter {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionTenant_Parameter { }

impl terrars::ToListMappable for CloudfrontDistributionTenant_Parameter {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Parameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionTenant_Parameter {
    pub fn build(self) -> CloudfrontDistributionTenant_Parameter {
        CloudfrontDistributionTenant_Parameter {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_ParameterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionTenant_ParameterRef {
        CloudfrontDistributionTenant_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_ParameterRef {
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

#[derive(Serialize)]
pub struct CloudfrontDistributionTenant_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionTenant_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionTenant_Timeouts {
    type O = terrars::BlockAssignable<CloudfrontDistributionTenant_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTenant_Timeouts {}

impl BuildCloudfrontDistributionTenant_Timeouts {
    pub fn build(self) -> CloudfrontDistributionTenant_Timeouts {
        CloudfrontDistributionTenant_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistributionTenant_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTenant_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionTenant_TimeoutsRef {
        CloudfrontDistributionTenant_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTenant_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
