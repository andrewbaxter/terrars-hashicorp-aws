use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectBotAssociationData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lex_bot: std::option::Option<std::vec::Vec<ConnectBotAssociationLexBotEl>>,
    dynamic: ConnectBotAssociationDynamic,
}

struct ConnectBotAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectBotAssociationData>,
}

#[derive(Clone)]
pub struct ConnectBotAssociation(std::rc::Rc<ConnectBotAssociation_>);

impl ConnectBotAssociation {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lex_bot`.\n"]
    pub fn set_lex_bot(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectBotAssociationLexBotEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lex_bot = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lex_bot = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lex_bot` after provisioning.\n"]
    pub fn lex_bot(&self) -> terrars::ListRef<ConnectBotAssociationLexBotElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lex_bot", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectBotAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectBotAssociation { }

impl terrars::ToListMappable for ConnectBotAssociation {
    type O = terrars::ListRef<ConnectBotAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectBotAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_bot_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectBotAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectBotAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectBotAssociation {
        let out = ConnectBotAssociation(std::rc::Rc::new(ConnectBotAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectBotAssociationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                lex_bot: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectBotAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectBotAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectBotAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lex_bot` after provisioning.\n"]
    pub fn lex_bot(&self) -> terrars::ListRef<ConnectBotAssociationLexBotElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lex_bot", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectBotAssociationLexBotEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lex_region: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl ConnectBotAssociationLexBotEl {
    #[doc = "Set the field `lex_region`.\n"]
    pub fn set_lex_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lex_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectBotAssociationLexBotEl {
    type O = terrars::BlockAssignable<ConnectBotAssociationLexBotEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectBotAssociationLexBotEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectBotAssociationLexBotElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectBotAssociationLexBotElRef {
        ConnectBotAssociationLexBotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectBotAssociationLexBotElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lex_region` after provisioning.\n"]
    pub fn lex_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lex_region", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectBotAssociationDynamic {
    lex_bot: std::option::Option<terrars::DynamicBlock<ConnectBotAssociationLexBotEl>>,
}
