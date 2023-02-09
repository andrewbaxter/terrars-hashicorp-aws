use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53KeySigningKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    hosted_zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_management_service_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

struct Route53KeySigningKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53KeySigningKeyData>,
}

#[derive(Clone)]
pub struct Route53KeySigningKey(Rc<Route53KeySigningKey_>);

impl Route53KeySigningKey {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `digest_algorithm_mnemonic` after provisioning.\n"]
    pub fn digest_algorithm_mnemonic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm_mnemonic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_algorithm_type` after provisioning.\n"]
    pub fn digest_algorithm_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_value` after provisioning.\n"]
    pub fn digest_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnskey_record` after provisioning.\n"]
    pub fn dnskey_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnskey_record", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ds_record` after provisioning.\n"]
    pub fn ds_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ds_record", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flag` after provisioning.\n"]
    pub fn flag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_management_service_arn` after provisioning.\n"]
    pub fn key_management_service_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_management_service_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signing_algorithm_mnemonic` after provisioning.\n"]
    pub fn signing_algorithm_mnemonic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm_mnemonic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signing_algorithm_type` after provisioning.\n"]
    pub fn signing_algorithm_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl Resource for Route53KeySigningKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Route53KeySigningKey {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Route53KeySigningKey {
    type O = ListRef<Route53KeySigningKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53KeySigningKey_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53_key_signing_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53KeySigningKey {
    pub tf_id: String,
    #[doc= ""]
    pub hosted_zone_id: PrimField<String>,
    #[doc= ""]
    pub key_management_service_arn: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildRoute53KeySigningKey {
    pub fn build(self, stack: &mut Stack) -> Route53KeySigningKey {
        let out = Route53KeySigningKey(Rc::new(Route53KeySigningKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53KeySigningKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                hosted_zone_id: self.hosted_zone_id,
                id: core::default::Default::default(),
                key_management_service_arn: self.key_management_service_arn,
                name: self.name,
                status: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53KeySigningKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53KeySigningKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53KeySigningKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `digest_algorithm_mnemonic` after provisioning.\n"]
    pub fn digest_algorithm_mnemonic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm_mnemonic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_algorithm_type` after provisioning.\n"]
    pub fn digest_algorithm_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_algorithm_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `digest_value` after provisioning.\n"]
    pub fn digest_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dnskey_record` after provisioning.\n"]
    pub fn dnskey_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnskey_record", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ds_record` after provisioning.\n"]
    pub fn ds_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ds_record", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flag` after provisioning.\n"]
    pub fn flag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_management_service_arn` after provisioning.\n"]
    pub fn key_management_service_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_management_service_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signing_algorithm_mnemonic` after provisioning.\n"]
    pub fn signing_algorithm_mnemonic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm_mnemonic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signing_algorithm_type` after provisioning.\n"]
    pub fn signing_algorithm_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}
