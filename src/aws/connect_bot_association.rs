use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectBotAssociationData {
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
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lex_bot: Option<Vec<ConnectBotAssociationLexBotEl>>,
    dynamic: ConnectBotAssociationDynamic,
}

struct ConnectBotAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectBotAssociationData>,
}

#[derive(Clone)]
pub struct ConnectBotAssociation(Rc<ConnectBotAssociation_>);

impl ConnectBotAssociation {
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

    #[doc= "Set the field `lex_bot`.\n"]
    pub fn set_lex_bot(self, v: impl Into<BlockAssignable<ConnectBotAssociationLexBotEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lex_bot = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lex_bot = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lex_bot` after provisioning.\n"]
    pub fn lex_bot(&self) -> ListRef<ConnectBotAssociationLexBotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lex_bot", self.extract_ref()))
    }
}

impl Resource for ConnectBotAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for ConnectBotAssociation {
    type O = ListRef<ConnectBotAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConnectBotAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_bot_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectBotAssociation {
    pub tf_id: String,
    #[doc= ""]
    pub instance_id: PrimField<String>,
}

impl BuildConnectBotAssociation {
    pub fn build(self, stack: &mut Stack) -> ConnectBotAssociation {
        let out = ConnectBotAssociation(Rc::new(ConnectBotAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectBotAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                lex_bot: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectBotAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectBotAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectBotAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lex_bot` after provisioning.\n"]
    pub fn lex_bot(&self) -> ListRef<ConnectBotAssociationLexBotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lex_bot", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectBotAssociationLexBotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lex_region: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl ConnectBotAssociationLexBotEl {
    #[doc= "Set the field `lex_region`.\n"]
    pub fn set_lex_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lex_region = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectBotAssociationLexBotEl {
    type O = BlockAssignable<ConnectBotAssociationLexBotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectBotAssociationLexBotEl {
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildConnectBotAssociationLexBotEl {
    pub fn build(self) -> ConnectBotAssociationLexBotEl {
        ConnectBotAssociationLexBotEl {
            lex_region: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct ConnectBotAssociationLexBotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectBotAssociationLexBotElRef {
    fn new(shared: StackShared, base: String) -> ConnectBotAssociationLexBotElRef {
        ConnectBotAssociationLexBotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectBotAssociationLexBotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lex_region` after provisioning.\n"]
    pub fn lex_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lex_region", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectBotAssociationDynamic {
    lex_bot: Option<DynamicBlock<ConnectBotAssociationLexBotEl>>,
}
