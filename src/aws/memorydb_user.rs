use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MemorydbUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_string: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    user_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_mode: Option<Vec<MemorydbUserAuthenticationModeEl>>,
    dynamic: MemorydbUserDynamic,
}

struct MemorydbUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MemorydbUserData>,
}

#[derive(Clone)]
pub struct MemorydbUser(Rc<MemorydbUser_>);

impl MemorydbUser {
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

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `authentication_mode`.\n"]
    pub fn set_authentication_mode(self, v: impl Into<BlockAssignable<MemorydbUserAuthenticationModeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authentication_mode = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authentication_mode = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `access_string` after provisioning.\n"]
    pub fn access_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_string", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimum_engine_version` after provisioning.\n"]
    pub fn minimum_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_engine_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authentication_mode` after provisioning.\n"]
    pub fn authentication_mode(&self) -> ListRef<MemorydbUserAuthenticationModeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authentication_mode", self.extract_ref()))
    }
}

impl Referable for MemorydbUser {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for MemorydbUser { }

impl ToListMappable for MemorydbUser {
    type O = ListRef<MemorydbUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for MemorydbUser_ {
    fn extract_resource_type(&self) -> String {
        "aws_memorydb_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMemorydbUser {
    pub tf_id: String,
    #[doc= ""]
    pub access_string: PrimField<String>,
    #[doc= ""]
    pub user_name: PrimField<String>,
}

impl BuildMemorydbUser {
    pub fn build(self, stack: &mut Stack) -> MemorydbUser {
        let out = MemorydbUser(Rc::new(MemorydbUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MemorydbUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_string: self.access_string,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_name: self.user_name,
                authentication_mode: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MemorydbUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for MemorydbUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MemorydbUserRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_string` after provisioning.\n"]
    pub fn access_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_string", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimum_engine_version` after provisioning.\n"]
    pub fn minimum_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_engine_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authentication_mode` after provisioning.\n"]
    pub fn authentication_mode(&self) -> ListRef<MemorydbUserAuthenticationModeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authentication_mode", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MemorydbUserAuthenticationModeEl {
    passwords: SetField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl MemorydbUserAuthenticationModeEl { }

impl ToListMappable for MemorydbUserAuthenticationModeEl {
    type O = BlockAssignable<MemorydbUserAuthenticationModeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorydbUserAuthenticationModeEl {
    #[doc= ""]
    pub passwords: SetField<PrimField<String>>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildMemorydbUserAuthenticationModeEl {
    pub fn build(self) -> MemorydbUserAuthenticationModeEl {
        MemorydbUserAuthenticationModeEl {
            passwords: self.passwords,
            type_: self.type_,
        }
    }
}

pub struct MemorydbUserAuthenticationModeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MemorydbUserAuthenticationModeElRef {
    fn new(shared: StackShared, base: String) -> MemorydbUserAuthenticationModeElRef {
        MemorydbUserAuthenticationModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorydbUserAuthenticationModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password_count` after provisioning.\n"]
    pub fn password_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_count", self.base))
    }

    #[doc= "Get a reference to the value of field `passwords` after provisioning.\n"]
    pub fn passwords(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.passwords", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorydbUserDynamic {
    authentication_mode: Option<DynamicBlock<MemorydbUserAuthenticationModeEl>>,
}
