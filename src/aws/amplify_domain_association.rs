use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AmplifyDomainAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_id: PrimField<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_verification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_domain: Option<Vec<AmplifyDomainAssociationSubDomainEl>>,
    dynamic: AmplifyDomainAssociationDynamic,
}

struct AmplifyDomainAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AmplifyDomainAssociationData>,
}

#[derive(Clone)]
pub struct AmplifyDomainAssociation(Rc<AmplifyDomainAssociation_>);

impl AmplifyDomainAssociation {
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

    #[doc= "Set the field `wait_for_verification`.\n"]
    pub fn set_wait_for_verification(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_verification = Some(v.into());
        self
    }

    #[doc= "Set the field `sub_domain`.\n"]
    pub fn set_sub_domain(self, v: impl Into<BlockAssignable<AmplifyDomainAssociationSubDomainEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sub_domain = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sub_domain = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_verification_dns_record` after provisioning.\n"]
    pub fn certificate_verification_dns_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_verification_dns_record", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_verification` after provisioning.\n"]
    pub fn wait_for_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_verification", self.extract_ref()))
    }
}

impl Resource for AmplifyDomainAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for AmplifyDomainAssociation {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for AmplifyDomainAssociation {
    type O = ListRef<AmplifyDomainAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AmplifyDomainAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_amplify_domain_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAmplifyDomainAssociation {
    pub tf_id: String,
    #[doc= ""]
    pub app_id: PrimField<String>,
    #[doc= ""]
    pub domain_name: PrimField<String>,
}

impl BuildAmplifyDomainAssociation {
    pub fn build(self, stack: &mut Stack) -> AmplifyDomainAssociation {
        let out = AmplifyDomainAssociation(Rc::new(AmplifyDomainAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AmplifyDomainAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_id: self.app_id,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                wait_for_verification: core::default::Default::default(),
                sub_domain: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AmplifyDomainAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyDomainAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AmplifyDomainAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_verification_dns_record` after provisioning.\n"]
    pub fn certificate_verification_dns_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_verification_dns_record", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_for_verification` after provisioning.\n"]
    pub fn wait_for_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_verification", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AmplifyDomainAssociationSubDomainEl {
    branch_name: PrimField<String>,
    prefix: PrimField<String>,
}

impl AmplifyDomainAssociationSubDomainEl { }

impl ToListMappable for AmplifyDomainAssociationSubDomainEl {
    type O = BlockAssignable<AmplifyDomainAssociationSubDomainEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyDomainAssociationSubDomainEl {
    #[doc= ""]
    pub branch_name: PrimField<String>,
    #[doc= ""]
    pub prefix: PrimField<String>,
}

impl BuildAmplifyDomainAssociationSubDomainEl {
    pub fn build(self) -> AmplifyDomainAssociationSubDomainEl {
        AmplifyDomainAssociationSubDomainEl {
            branch_name: self.branch_name,
            prefix: self.prefix,
        }
    }
}

pub struct AmplifyDomainAssociationSubDomainElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyDomainAssociationSubDomainElRef {
    fn new(shared: StackShared, base: String) -> AmplifyDomainAssociationSubDomainElRef {
        AmplifyDomainAssociationSubDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyDomainAssociationSubDomainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc= "Get a reference to the value of field `dns_record` after provisioning.\n"]
    pub fn dns_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_record", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `verified` after provisioning.\n"]
    pub fn verified(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verified", self.base))
    }
}

#[derive(Serialize, Default)]
struct AmplifyDomainAssociationDynamic {
    sub_domain: Option<DynamicBlock<AmplifyDomainAssociationSubDomainEl>>,
}
