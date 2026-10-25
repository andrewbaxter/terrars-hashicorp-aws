#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53domainsDomainData_ {
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
    admin_privacy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_renew: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_contact: std::option::Option<terrars::ListField<Route53domainsDomain_BillingContact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_privacy: std::option::Option<terrars::PrimField<bool>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration_in_years: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_server: std::option::Option<terrars::ListField<Route53domainsDomain_NameServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registrant_privacy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tech_privacy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_lock: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_contact: std::option::Option<std::vec::Vec<Route53domainsDomain_AdminContact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registrant_contact: std::option::Option<std::vec::Vec<Route53domainsDomain_RegistrantContact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tech_contact: std::option::Option<std::vec::Vec<Route53domainsDomain_TechContact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Route53domainsDomain_Timeouts>,
    dynamic: Route53domainsDomainDynamic,
}

struct Route53domainsDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53domainsDomainData_>,
}

#[derive(Clone)]
pub struct Route53domainsDomain(std::rc::Rc<Route53domainsDomain_>);

impl Route53domainsDomain {
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

    #[doc = "Set the field `admin_privacy`.\n"]
    pub fn set_admin_privacy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_privacy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_renew`.\n"]
    pub fn set_auto_renew(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_renew = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_contact`.\n"]
    pub fn set_billing_contact(
        self,
        v: impl std::convert::Into<terrars::ListField<Route53domainsDomain_BillingContact>>,
    ) -> Self {
        self.0.data.borrow_mut().billing_contact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_privacy`.\n"]
    pub fn set_billing_privacy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().billing_privacy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration_in_years`.\n"]
    pub fn set_duration_in_years(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().duration_in_years = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_server`.\n"]
    pub fn set_name_server(
        self,
        v: impl std::convert::Into<terrars::ListField<Route53domainsDomain_NameServer>>,
    ) -> Self {
        self.0.data.borrow_mut().name_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registrant_privacy`.\n"]
    pub fn set_registrant_privacy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().registrant_privacy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tech_privacy`.\n"]
    pub fn set_tech_privacy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tech_privacy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_lock`.\n"]
    pub fn set_transfer_lock(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transfer_lock = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_contact`.\n"]
    pub fn set_admin_contact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsDomain_AdminContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_contact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registrant_contact`.\n"]
    pub fn set_registrant_contact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsDomain_RegistrantContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().registrant_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.registrant_contact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tech_contact`.\n"]
    pub fn set_tech_contact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsDomain_TechContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tech_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tech_contact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Route53domainsDomain_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `abuse_contact_email` after provisioning.\n"]
    pub fn abuse_contact_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abuse_contact_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `abuse_contact_phone` after provisioning.\n"]
    pub fn abuse_contact_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abuse_contact_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_privacy` after provisioning.\n"]
    pub fn admin_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\n"]
    pub fn auto_renew(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_renew", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_contact` after provisioning.\n"]
    pub fn billing_contact(&self) -> terrars::ListRef<Route53domainsDomain_BillingContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_privacy` after provisioning.\n"]
    pub fn billing_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `duration_in_years` after provisioning.\n"]
    pub fn duration_in_years(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_years", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> terrars::ListRef<Route53domainsDomain_NameServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrant_privacy` after provisioning.\n"]
    pub fn registrant_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registrant_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrar_name` after provisioning.\n"]
    pub fn registrar_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registrar_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrar_url` after provisioning.\n"]
    pub fn registrar_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registrar_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_list` after provisioning.\n"]
    pub fn status_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tech_privacy` after provisioning.\n"]
    pub fn tech_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tech_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transfer_lock` after provisioning.\n"]
    pub fn transfer_lock(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_lock", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_date` after provisioning.\n"]
    pub fn updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `whois_server` after provisioning.\n"]
    pub fn whois_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.whois_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_contact` after provisioning.\n"]
    pub fn admin_contact(&self) -> terrars::ListRef<Route53domainsDomain_AdminContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> terrars::ListRef<Route53domainsDomain_RegistrantContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registrant_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> terrars::ListRef<Route53domainsDomain_TechContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tech_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsDomain_TimeoutsRef {
        Route53domainsDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Route53domainsDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Route53domainsDomain { }

impl terrars::ToListMappable for Route53domainsDomain {
    type O = terrars::ListRef<Route53domainsDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Route53domainsDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_route53domains_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53domainsDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53domainsDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> Route53domainsDomain {
        let out = Route53domainsDomain(std::rc::Rc::new(Route53domainsDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Route53domainsDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_privacy: core::default::Default::default(),
                auto_renew: core::default::Default::default(),
                billing_contact: core::default::Default::default(),
                billing_privacy: core::default::Default::default(),
                domain_name: self.domain_name,
                duration_in_years: core::default::Default::default(),
                name_server: core::default::Default::default(),
                registrant_privacy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tech_privacy: core::default::Default::default(),
                transfer_lock: core::default::Default::default(),
                admin_contact: core::default::Default::default(),
                registrant_contact: core::default::Default::default(),
                tech_contact: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53domainsDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53domainsDomainRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abuse_contact_email` after provisioning.\n"]
    pub fn abuse_contact_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abuse_contact_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `abuse_contact_phone` after provisioning.\n"]
    pub fn abuse_contact_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abuse_contact_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_privacy` after provisioning.\n"]
    pub fn admin_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\n"]
    pub fn auto_renew(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_renew", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_contact` after provisioning.\n"]
    pub fn billing_contact(&self) -> terrars::ListRef<Route53domainsDomain_BillingContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_privacy` after provisioning.\n"]
    pub fn billing_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `duration_in_years` after provisioning.\n"]
    pub fn duration_in_years(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_years", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> terrars::ListRef<Route53domainsDomain_NameServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrant_privacy` after provisioning.\n"]
    pub fn registrant_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registrant_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrar_name` after provisioning.\n"]
    pub fn registrar_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registrar_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrar_url` after provisioning.\n"]
    pub fn registrar_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registrar_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_list` after provisioning.\n"]
    pub fn status_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tech_privacy` after provisioning.\n"]
    pub fn tech_privacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tech_privacy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transfer_lock` after provisioning.\n"]
    pub fn transfer_lock(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_lock", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_date` after provisioning.\n"]
    pub fn updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `whois_server` after provisioning.\n"]
    pub fn whois_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.whois_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_contact` after provisioning.\n"]
    pub fn admin_contact(&self) -> terrars::ListRef<Route53domainsDomain_AdminContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> terrars::ListRef<Route53domainsDomain_RegistrantContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registrant_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> terrars::ListRef<Route53domainsDomain_TechContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tech_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsDomain_TimeoutsRef {
        Route53domainsDomain_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Route53domainsDomainDynamic {
    admin_contact: std::option::Option<terrars::DynamicBlock<Route53domainsDomain_AdminContact>>,
    registrant_contact: std::option::Option<terrars::DynamicBlock<Route53domainsDomain_RegistrantContact>>,
    tech_contact: std::option::Option<terrars::DynamicBlock<Route53domainsDomain_TechContact>>,
}

#[derive(Serialize)]
pub struct Route53domainsDomain_BillingContact_ExtraParam {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53domainsDomain_BillingContact_ExtraParam {
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

impl terrars::ToListMappable for Route53domainsDomain_BillingContact_ExtraParam {
    type O = terrars::BlockAssignable<Route53domainsDomain_BillingContact_ExtraParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_BillingContact_ExtraParam {}

impl BuildRoute53domainsDomain_BillingContact_ExtraParam {
    pub fn build(self) -> Route53domainsDomain_BillingContact_ExtraParam {
        Route53domainsDomain_BillingContact_ExtraParam {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_BillingContact_ExtraParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_BillingContact_ExtraParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53domainsDomain_BillingContact_ExtraParamRef {
        Route53domainsDomain_BillingContact_ExtraParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_BillingContact_ExtraParamRef {
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
pub struct Route53domainsDomain_BillingContact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_param: std::option::Option<terrars::ListField<Route53domainsDomain_BillingContact_ExtraParam>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53domainsDomain_BillingContact {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Route53domainsDomain_BillingContact_ExtraParam>>,
    ) -> Self {
        self.extra_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zip_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53domainsDomain_BillingContact {
    type O = terrars::BlockAssignable<Route53domainsDomain_BillingContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_BillingContact {}

impl BuildRoute53domainsDomain_BillingContact {
    pub fn build(self) -> Route53domainsDomain_BillingContact {
        Route53domainsDomain_BillingContact {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_BillingContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_BillingContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsDomain_BillingContactRef {
        Route53domainsDomain_BillingContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_BillingContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_2", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> terrars::ListRef<Route53domainsDomain_BillingContact_ExtraParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }

    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53domainsDomain_NameServer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53domainsDomain_NameServer {
    #[doc = "Set the field `glue_ips`.\n"]
    pub fn set_glue_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.glue_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53domainsDomain_NameServer {
    type O = terrars::BlockAssignable<Route53domainsDomain_NameServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_NameServer {}

impl BuildRoute53domainsDomain_NameServer {
    pub fn build(self) -> Route53domainsDomain_NameServer {
        Route53domainsDomain_NameServer {
            glue_ips: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_NameServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_NameServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsDomain_NameServerRef {
        Route53domainsDomain_NameServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_NameServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `glue_ips` after provisioning.\n"]
    pub fn glue_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.glue_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53domainsDomain_TechContact_ExtraParam {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Route53domainsDomain_TechContact_ExtraParam { }

impl terrars::ToListMappable for Route53domainsDomain_TechContact_ExtraParam {
    type O = terrars::BlockAssignable<Route53domainsDomain_TechContact_ExtraParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_TechContact_ExtraParam {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildRoute53domainsDomain_TechContact_ExtraParam {
    pub fn build(self) -> Route53domainsDomain_TechContact_ExtraParam {
        Route53domainsDomain_TechContact_ExtraParam {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_TechContact_ExtraParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_TechContact_ExtraParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53domainsDomain_TechContact_ExtraParamRef {
        Route53domainsDomain_TechContact_ExtraParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_TechContact_ExtraParamRef {
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

#[derive(Serialize, Default)]
struct Route53domainsDomain_AdminContactDynamic {
    extra_param: std::option::Option<terrars::DynamicBlock<Route53domainsDomain_TechContact_ExtraParam>>,
}

#[derive(Serialize)]
pub struct Route53domainsDomain_AdminContact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_param: std::option::Option<std::vec::Vec<Route53domainsDomain_TechContact_ExtraParam>>,
    dynamic: Route53domainsDomain_AdminContactDynamic,
}

impl Route53domainsDomain_AdminContact {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zip_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsDomain_TechContact_ExtraParam>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extra_param = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extra_param = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53domainsDomain_AdminContact {
    type O = terrars::BlockAssignable<Route53domainsDomain_AdminContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_AdminContact {}

impl BuildRoute53domainsDomain_AdminContact {
    pub fn build(self) -> Route53domainsDomain_AdminContact {
        Route53domainsDomain_AdminContact {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_AdminContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_AdminContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsDomain_AdminContactRef {
        Route53domainsDomain_AdminContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_AdminContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_2", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }

    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> terrars::ListRef<Route53domainsDomain_TechContact_ExtraParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53domainsDomain_RegistrantContactDynamic {
    extra_param: std::option::Option<terrars::DynamicBlock<Route53domainsDomain_TechContact_ExtraParam>>,
}

#[derive(Serialize)]
pub struct Route53domainsDomain_RegistrantContact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_param: std::option::Option<std::vec::Vec<Route53domainsDomain_TechContact_ExtraParam>>,
    dynamic: Route53domainsDomain_RegistrantContactDynamic,
}

impl Route53domainsDomain_RegistrantContact {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zip_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsDomain_TechContact_ExtraParam>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extra_param = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extra_param = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53domainsDomain_RegistrantContact {
    type O = terrars::BlockAssignable<Route53domainsDomain_RegistrantContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_RegistrantContact {}

impl BuildRoute53domainsDomain_RegistrantContact {
    pub fn build(self) -> Route53domainsDomain_RegistrantContact {
        Route53domainsDomain_RegistrantContact {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_RegistrantContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_RegistrantContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsDomain_RegistrantContactRef {
        Route53domainsDomain_RegistrantContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_RegistrantContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_2", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }

    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> terrars::ListRef<Route53domainsDomain_TechContact_ExtraParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53domainsDomain_TechContactDynamic {
    extra_param: std::option::Option<terrars::DynamicBlock<Route53domainsDomain_TechContact_ExtraParam>>,
}

#[derive(Serialize)]
pub struct Route53domainsDomain_TechContact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line_2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_param: std::option::Option<std::vec::Vec<Route53domainsDomain_TechContact_ExtraParam>>,
    dynamic: Route53domainsDomain_TechContactDynamic,
}

impl Route53domainsDomain_TechContact {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_line_2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zip_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsDomain_TechContact_ExtraParam>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extra_param = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extra_param = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53domainsDomain_TechContact {
    type O = terrars::BlockAssignable<Route53domainsDomain_TechContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_TechContact {}

impl BuildRoute53domainsDomain_TechContact {
    pub fn build(self) -> Route53domainsDomain_TechContact {
        Route53domainsDomain_TechContact {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_TechContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_TechContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsDomain_TechContactRef {
        Route53domainsDomain_TechContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_TechContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_1", self.base))
    }

    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_2", self.base))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }

    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> terrars::ListRef<Route53domainsDomain_TechContact_ExtraParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53domainsDomain_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53domainsDomain_Timeouts {
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

impl terrars::ToListMappable for Route53domainsDomain_Timeouts {
    type O = terrars::BlockAssignable<Route53domainsDomain_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDomain_Timeouts {}

impl BuildRoute53domainsDomain_Timeouts {
    pub fn build(self) -> Route53domainsDomain_Timeouts {
        Route53domainsDomain_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53domainsDomain_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsDomain_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsDomain_TimeoutsRef {
        Route53domainsDomain_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDomain_TimeoutsRef {
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
