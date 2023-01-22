use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53ResolverRuleData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolver_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolver_rule_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataRoute53ResolverRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRoute53ResolverRuleData>,
}

#[derive(Clone)]
pub struct DataRoute53ResolverRule(Rc<DataRoute53ResolverRule_>);

impl DataRoute53ResolverRule {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `domain_name`.\n"]
    pub fn set_domain_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `resolver_endpoint_id`.\n"]
    pub fn set_resolver_endpoint_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resolver_endpoint_id = Some(v.into());
        self
    }

    #[doc= "Set the field `resolver_rule_id`.\n"]
    pub fn set_resolver_rule_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resolver_rule_id = Some(v.into());
        self
    }

    #[doc= "Set the field `rule_type`.\n"]
    pub fn set_rule_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rule_type = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resolver_endpoint_id` after provisioning.\n"]
    pub fn resolver_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolver_endpoint_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resolver_rule_id` after provisioning.\n"]
    pub fn resolver_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolver_rule_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_type` after provisioning.\n"]
    pub fn rule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_status` after provisioning.\n"]
    pub fn share_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Datasource for DataRoute53ResolverRule {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataRoute53ResolverRule {
    type O = ListRef<DataRoute53ResolverRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRoute53ResolverRule_ {
    fn extract_datasource_type(&self) -> String {
        "aws_route53_resolver_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53ResolverRule {
    pub tf_id: String,
}

impl BuildDataRoute53ResolverRule {
    pub fn build(self, stack: &mut Stack) -> DataRoute53ResolverRule {
        let out = DataRoute53ResolverRule(Rc::new(DataRoute53ResolverRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRoute53ResolverRuleData {
                provider: None,
                for_each: None,
                domain_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                resolver_endpoint_id: core::default::Default::default(),
                resolver_rule_id: core::default::Default::default(),
                rule_type: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53ResolverRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRoute53ResolverRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRoute53ResolverRuleRef {
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

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resolver_endpoint_id` after provisioning.\n"]
    pub fn resolver_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolver_endpoint_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resolver_rule_id` after provisioning.\n"]
    pub fn resolver_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolver_rule_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_type` after provisioning.\n"]
    pub fn rule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_status` after provisioning.\n"]
    pub fn share_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}
