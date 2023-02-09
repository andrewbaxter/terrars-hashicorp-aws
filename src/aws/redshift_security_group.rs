use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftSecurityGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<Vec<RedshiftSecurityGroupIngressEl>>,
    dynamic: RedshiftSecurityGroupDynamic,
}

struct RedshiftSecurityGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftSecurityGroupData>,
}

#[derive(Clone)]
pub struct RedshiftSecurityGroup(Rc<RedshiftSecurityGroup_>);

impl RedshiftSecurityGroup {
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

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ingress`.\n"]
    pub fn set_ingress(self, v: impl Into<BlockAssignable<RedshiftSecurityGroupIngressEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Resource for RedshiftSecurityGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for RedshiftSecurityGroup {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for RedshiftSecurityGroup {
    type O = ListRef<RedshiftSecurityGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RedshiftSecurityGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshift_security_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftSecurityGroup {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildRedshiftSecurityGroup {
    pub fn build(self, stack: &mut Stack) -> RedshiftSecurityGroup {
        let out = RedshiftSecurityGroup(Rc::new(RedshiftSecurityGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftSecurityGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                ingress: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftSecurityGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftSecurityGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftSecurityGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftSecurityGroupIngressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_owner_id: Option<PrimField<String>>,
}

impl RedshiftSecurityGroupIngressEl {
    #[doc= "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc= "Set the field `security_group_name`.\n"]
    pub fn set_security_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `security_group_owner_id`.\n"]
    pub fn set_security_group_owner_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group_owner_id = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftSecurityGroupIngressEl {
    type O = BlockAssignable<RedshiftSecurityGroupIngressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftSecurityGroupIngressEl {}

impl BuildRedshiftSecurityGroupIngressEl {
    pub fn build(self) -> RedshiftSecurityGroupIngressEl {
        RedshiftSecurityGroupIngressEl {
            cidr: core::default::Default::default(),
            security_group_name: core::default::Default::default(),
            security_group_owner_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftSecurityGroupIngressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftSecurityGroupIngressElRef {
    fn new(shared: StackShared, base: String) -> RedshiftSecurityGroupIngressElRef {
        RedshiftSecurityGroupIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftSecurityGroupIngressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc= "Get a reference to the value of field `security_group_name` after provisioning.\n"]
    pub fn security_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `security_group_owner_id` after provisioning.\n"]
    pub fn security_group_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_owner_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftSecurityGroupDynamic {
    ingress: Option<DynamicBlock<RedshiftSecurityGroupIngressEl>>,
}
