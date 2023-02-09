use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53ResolverFirewallDomainListData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    firewall_domain_list_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataRoute53ResolverFirewallDomainList_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRoute53ResolverFirewallDomainListData>,
}

#[derive(Clone)]
pub struct DataRoute53ResolverFirewallDomainList(Rc<DataRoute53ResolverFirewallDomainList_>);

impl DataRoute53ResolverFirewallDomainList {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creator_request_id` after provisioning.\n"]
    pub fn creator_request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creator_request_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_count` after provisioning.\n"]
    pub fn domain_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_owner_name` after provisioning.\n"]
    pub fn managed_owner_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_owner_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modification_time` after provisioning.\n"]
    pub fn modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modification_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }
}

impl Datasource for DataRoute53ResolverFirewallDomainList {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataRoute53ResolverFirewallDomainList {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataRoute53ResolverFirewallDomainList {
    type O = ListRef<DataRoute53ResolverFirewallDomainListRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRoute53ResolverFirewallDomainList_ {
    fn extract_datasource_type(&self) -> String {
        "aws_route53_resolver_firewall_domain_list".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53ResolverFirewallDomainList {
    pub tf_id: String,
    #[doc= ""]
    pub firewall_domain_list_id: PrimField<String>,
}

impl BuildDataRoute53ResolverFirewallDomainList {
    pub fn build(self, stack: &mut Stack) -> DataRoute53ResolverFirewallDomainList {
        let out = DataRoute53ResolverFirewallDomainList(Rc::new(DataRoute53ResolverFirewallDomainList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRoute53ResolverFirewallDomainListData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                firewall_domain_list_id: self.firewall_domain_list_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53ResolverFirewallDomainListRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRoute53ResolverFirewallDomainListRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRoute53ResolverFirewallDomainListRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creator_request_id` after provisioning.\n"]
    pub fn creator_request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creator_request_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_count` after provisioning.\n"]
    pub fn domain_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_domain_list_id` after provisioning.\n"]
    pub fn firewall_domain_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_domain_list_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed_owner_name` after provisioning.\n"]
    pub fn managed_owner_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_owner_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modification_time` after provisioning.\n"]
    pub fn modification_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modification_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }
}
