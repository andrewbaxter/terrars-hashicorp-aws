use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OrganizationsDelegatedAdministratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    service_principal: PrimField<String>,
}

struct OrganizationsDelegatedAdministrator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationsDelegatedAdministratorData>,
}

#[derive(Clone)]
pub struct OrganizationsDelegatedAdministrator(Rc<OrganizationsDelegatedAdministrator_>);

impl OrganizationsDelegatedAdministrator {
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

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delegation_enabled_date` after provisioning.\n"]
    pub fn delegation_enabled_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delegation_enabled_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_principal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl Resource for OrganizationsDelegatedAdministrator {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for OrganizationsDelegatedAdministrator {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for OrganizationsDelegatedAdministrator {
    type O = ListRef<OrganizationsDelegatedAdministratorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for OrganizationsDelegatedAdministrator_ {
    fn extract_resource_type(&self) -> String {
        "aws_organizations_delegated_administrator".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationsDelegatedAdministrator {
    pub tf_id: String,
    #[doc= ""]
    pub account_id: PrimField<String>,
    #[doc= ""]
    pub service_principal: PrimField<String>,
}

impl BuildOrganizationsDelegatedAdministrator {
    pub fn build(self, stack: &mut Stack) -> OrganizationsDelegatedAdministrator {
        let out = OrganizationsDelegatedAdministrator(Rc::new(OrganizationsDelegatedAdministrator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrganizationsDelegatedAdministratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                service_principal: self.service_principal,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationsDelegatedAdministratorRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationsDelegatedAdministratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationsDelegatedAdministratorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delegation_enabled_date` after provisioning.\n"]
    pub fn delegation_enabled_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delegation_enabled_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_principal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}
