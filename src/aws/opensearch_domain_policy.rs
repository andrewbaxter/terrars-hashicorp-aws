use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchDomainPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_policies: PrimField<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OpensearchDomainPolicyTimeoutsEl>,
}

struct OpensearchDomainPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpensearchDomainPolicyData>,
}

#[derive(Clone)]
pub struct OpensearchDomainPolicy(Rc<OpensearchDomainPolicy_>);

impl OpensearchDomainPolicy {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OpensearchDomainPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchDomainPolicyTimeoutsElRef {
        OpensearchDomainPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for OpensearchDomainPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for OpensearchDomainPolicy {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for OpensearchDomainPolicy {
    type O = ListRef<OpensearchDomainPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for OpensearchDomainPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_opensearch_domain_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchDomainPolicy {
    pub tf_id: String,
    #[doc= ""]
    pub access_policies: PrimField<String>,
    #[doc= ""]
    pub domain_name: PrimField<String>,
}

impl BuildOpensearchDomainPolicy {
    pub fn build(self, stack: &mut Stack) -> OpensearchDomainPolicy {
        let out = OpensearchDomainPolicy(Rc::new(OpensearchDomainPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpensearchDomainPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_policies: self.access_policies,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchDomainPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchDomainPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchDomainPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchDomainPolicyTimeoutsElRef {
        OpensearchDomainPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpensearchDomainPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl OpensearchDomainPolicyTimeoutsEl {
    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for OpensearchDomainPolicyTimeoutsEl {
    type O = BlockAssignable<OpensearchDomainPolicyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchDomainPolicyTimeoutsEl {}

impl BuildOpensearchDomainPolicyTimeoutsEl {
    pub fn build(self) -> OpensearchDomainPolicyTimeoutsEl {
        OpensearchDomainPolicyTimeoutsEl {
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OpensearchDomainPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchDomainPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainPolicyTimeoutsElRef {
        OpensearchDomainPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchDomainPolicyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
