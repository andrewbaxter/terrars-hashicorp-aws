use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksRdsDbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    db_password: PrimField<String>,
    db_user: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    rds_db_instance_arn: PrimField<String>,
    stack_id: PrimField<String>,
}

struct OpsworksRdsDbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpsworksRdsDbInstanceData>,
}

#[derive(Clone)]
pub struct OpsworksRdsDbInstance(Rc<OpsworksRdsDbInstance_>);

impl OpsworksRdsDbInstance {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Get a reference to the value of field `db_password` after provisioning.\n"]
    pub fn db_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rds_db_instance_arn` after provisioning.\n"]
    pub fn rds_db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rds_db_instance_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }
}

impl Resource for OpsworksRdsDbInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for OpsworksRdsDbInstance {
    type O = ListRef<OpsworksRdsDbInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OpsworksRdsDbInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_opsworks_rds_db_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksRdsDbInstance {
    pub tf_id: String,
    #[doc= ""]
    pub db_password: PrimField<String>,
    #[doc= ""]
    pub db_user: PrimField<String>,
    #[doc= ""]
    pub rds_db_instance_arn: PrimField<String>,
    #[doc= ""]
    pub stack_id: PrimField<String>,
}

impl BuildOpsworksRdsDbInstance {
    pub fn build(self, stack: &mut Stack) -> OpsworksRdsDbInstance {
        let out = OpsworksRdsDbInstance(Rc::new(OpsworksRdsDbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpsworksRdsDbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                db_password: self.db_password,
                db_user: self.db_user,
                id: core::default::Default::default(),
                rds_db_instance_arn: self.rds_db_instance_arn,
                stack_id: self.stack_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksRdsDbInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksRdsDbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksRdsDbInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `db_password` after provisioning.\n"]
    pub fn db_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rds_db_instance_arn` after provisioning.\n"]
    pub fn rds_db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rds_db_instance_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }
}
