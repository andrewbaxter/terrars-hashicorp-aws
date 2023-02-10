use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ServicequotasServiceQuotaData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    quota_code: PrimField<String>,
    service_code: PrimField<String>,
    value: PrimField<f64>,
}

struct ServicequotasServiceQuota_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServicequotasServiceQuotaData>,
}

#[derive(Clone)]
pub struct ServicequotasServiceQuota(Rc<ServicequotasServiceQuota_>);

impl ServicequotasServiceQuota {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_id` after provisioning.\n"]
    pub fn request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_status` after provisioning.\n"]
    pub fn request_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl Referable for ServicequotasServiceQuota {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ServicequotasServiceQuota { }

impl ToListMappable for ServicequotasServiceQuota {
    type O = ListRef<ServicequotasServiceQuotaRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServicequotasServiceQuota_ {
    fn extract_resource_type(&self) -> String {
        "aws_servicequotas_service_quota".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServicequotasServiceQuota {
    pub tf_id: String,
    #[doc= ""]
    pub quota_code: PrimField<String>,
    #[doc= ""]
    pub service_code: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<f64>,
}

impl BuildServicequotasServiceQuota {
    pub fn build(self, stack: &mut Stack) -> ServicequotasServiceQuota {
        let out = ServicequotasServiceQuota(Rc::new(ServicequotasServiceQuota_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServicequotasServiceQuotaData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                quota_code: self.quota_code,
                service_code: self.service_code,
                value: self.value,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServicequotasServiceQuotaRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicequotasServiceQuotaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServicequotasServiceQuotaRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_id` after provisioning.\n"]
    pub fn request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_status` after provisioning.\n"]
    pub fn request_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}
