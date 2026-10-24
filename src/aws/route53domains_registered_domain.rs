use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53domainsRegisteredDomainData {
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
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registrant_privacy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tech_privacy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_lock: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_contact: std::option::Option<std::vec::Vec<Route53domainsRegisteredDomainAdminContactEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_server: std::option::Option<std::vec::Vec<Route53domainsRegisteredDomainNameServerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registrant_contact: std::option::Option<std::vec::Vec<Route53domainsRegisteredDomainRegistrantContactEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tech_contact: std::option::Option<std::vec::Vec<Route53domainsRegisteredDomainTechContactEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Route53domainsRegisteredDomainTimeoutsEl>,
    dynamic: Route53domainsRegisteredDomainDynamic,
}

struct Route53domainsRegisteredDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53domainsRegisteredDomainData>,
}

#[derive(Clone)]
pub struct Route53domainsRegisteredDomain(std::rc::Rc<Route53domainsRegisteredDomain_>);

impl Route53domainsRegisteredDomain {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsRegisteredDomainAdminContactEl>>,
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

    #[doc = "Set the field `name_server`.\n"]
    pub fn set_name_server(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsRegisteredDomainNameServerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().name_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.name_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registrant_contact`.\n"]
    pub fn set_registrant_contact(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsRegisteredDomainRegistrantContactEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Route53domainsRegisteredDomainTechContactEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<Route53domainsRegisteredDomainTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `reseller` after provisioning.\n"]
    pub fn reseller(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reseller", self.extract_ref()))
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
    pub fn admin_contact(&self) -> terrars::ListRef<Route53domainsRegisteredDomainAdminContactElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> terrars::ListRef<Route53domainsRegisteredDomainNameServerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> terrars::ListRef<Route53domainsRegisteredDomainRegistrantContactElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registrant_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> terrars::ListRef<Route53domainsRegisteredDomainTechContactElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tech_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsRegisteredDomainTimeoutsElRef {
        Route53domainsRegisteredDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Route53domainsRegisteredDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Route53domainsRegisteredDomain { }

impl terrars::ToListMappable for Route53domainsRegisteredDomain {
    type O = terrars::ListRef<Route53domainsRegisteredDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Route53domainsRegisteredDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_route53domains_registered_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53domainsRegisteredDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53domainsRegisteredDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> Route53domainsRegisteredDomain {
        let out = Route53domainsRegisteredDomain(std::rc::Rc::new(Route53domainsRegisteredDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Route53domainsRegisteredDomainData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_privacy: core::default::Default::default(),
                auto_renew: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                registrant_privacy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tech_privacy: core::default::Default::default(),
                transfer_lock: core::default::Default::default(),
                admin_contact: core::default::Default::default(),
                name_server: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsRegisteredDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53domainsRegisteredDomainRef {
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

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `reseller` after provisioning.\n"]
    pub fn reseller(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reseller", self.extract_ref()))
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
    pub fn admin_contact(&self) -> terrars::ListRef<Route53domainsRegisteredDomainAdminContactElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> terrars::ListRef<Route53domainsRegisteredDomainNameServerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> terrars::ListRef<Route53domainsRegisteredDomainRegistrantContactElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registrant_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> terrars::ListRef<Route53domainsRegisteredDomainTechContactElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tech_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsRegisteredDomainTimeoutsElRef {
        Route53domainsRegisteredDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Route53domainsRegisteredDomainAdminContactEl {
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
    extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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

impl Route53domainsRegisteredDomainAdminContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extra_params = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for Route53domainsRegisteredDomainAdminContactEl {
    type O = terrars::BlockAssignable<Route53domainsRegisteredDomainAdminContactEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainAdminContactEl {}

impl BuildRoute53domainsRegisteredDomainAdminContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainAdminContactEl {
        Route53domainsRegisteredDomainAdminContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainAdminContactElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsRegisteredDomainAdminContactElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53domainsRegisteredDomainAdminContactElRef {
        Route53domainsRegisteredDomainAdminContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainAdminContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainNameServerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
}

impl Route53domainsRegisteredDomainNameServerEl {
    #[doc = "Set the field `glue_ips`.\n"]
    pub fn set_glue_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.glue_ips = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53domainsRegisteredDomainNameServerEl {
    type O = terrars::BlockAssignable<Route53domainsRegisteredDomainNameServerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainNameServerEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53domainsRegisteredDomainNameServerEl {
    pub fn build(self) -> Route53domainsRegisteredDomainNameServerEl {
        Route53domainsRegisteredDomainNameServerEl {
            glue_ips: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct Route53domainsRegisteredDomainNameServerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsRegisteredDomainNameServerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsRegisteredDomainNameServerElRef {
        Route53domainsRegisteredDomainNameServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainNameServerElRef {
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
pub struct Route53domainsRegisteredDomainRegistrantContactEl {
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
    extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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

impl Route53domainsRegisteredDomainRegistrantContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extra_params = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for Route53domainsRegisteredDomainRegistrantContactEl {
    type O = terrars::BlockAssignable<Route53domainsRegisteredDomainRegistrantContactEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainRegistrantContactEl {}

impl BuildRoute53domainsRegisteredDomainRegistrantContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainRegistrantContactEl {
        Route53domainsRegisteredDomainRegistrantContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainRegistrantContactElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsRegisteredDomainRegistrantContactElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53domainsRegisteredDomainRegistrantContactElRef {
        Route53domainsRegisteredDomainRegistrantContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainRegistrantContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainTechContactEl {
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
    extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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

impl Route53domainsRegisteredDomainTechContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extra_params = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for Route53domainsRegisteredDomainTechContactEl {
    type O = terrars::BlockAssignable<Route53domainsRegisteredDomainTechContactEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainTechContactEl {}

impl BuildRoute53domainsRegisteredDomainTechContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainTechContactEl {
        Route53domainsRegisteredDomainTechContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainTechContactElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsRegisteredDomainTechContactElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53domainsRegisteredDomainTechContactElRef {
        Route53domainsRegisteredDomainTechContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainTechContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53domainsRegisteredDomainTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53domainsRegisteredDomainTimeoutsEl {
    type O = terrars::BlockAssignable<Route53domainsRegisteredDomainTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainTimeoutsEl {}

impl BuildRoute53domainsRegisteredDomainTimeoutsEl {
    pub fn build(self) -> Route53domainsRegisteredDomainTimeoutsEl {
        Route53domainsRegisteredDomainTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Route53domainsRegisteredDomainTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53domainsRegisteredDomainTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53domainsRegisteredDomainTimeoutsElRef {
        Route53domainsRegisteredDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53domainsRegisteredDomainDynamic {
    admin_contact: std::option::Option<terrars::DynamicBlock<Route53domainsRegisteredDomainAdminContactEl>>,
    name_server: std::option::Option<terrars::DynamicBlock<Route53domainsRegisteredDomainNameServerEl>>,
    registrant_contact: std
    ::option
    ::Option<terrars::DynamicBlock<Route53domainsRegisteredDomainRegistrantContactEl>>,
    tech_contact: std::option::Option<terrars::DynamicBlock<Route53domainsRegisteredDomainTechContactEl>>,
}
