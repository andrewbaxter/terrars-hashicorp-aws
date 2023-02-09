use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ControltowerControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    control_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    target_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ControltowerControlTimeoutsEl>,
}

struct ControltowerControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ControltowerControlData>,
}

#[derive(Clone)]
pub struct ControltowerControl(Rc<ControltowerControl_>);

impl ControltowerControl {
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
    pub fn set_timeouts(self, v: impl Into<ControltowerControlTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `control_identifier` after provisioning.\n"]
    pub fn control_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_identifier` after provisioning.\n"]
    pub fn target_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ControltowerControlTimeoutsElRef {
        ControltowerControlTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for ControltowerControl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ControltowerControl {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ControltowerControl {
    type O = ListRef<ControltowerControlRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ControltowerControl_ {
    fn extract_resource_type(&self) -> String {
        "aws_controltower_control".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildControltowerControl {
    pub tf_id: String,
    #[doc= ""]
    pub control_identifier: PrimField<String>,
    #[doc= ""]
    pub target_identifier: PrimField<String>,
}

impl BuildControltowerControl {
    pub fn build(self, stack: &mut Stack) -> ControltowerControl {
        let out = ControltowerControl(Rc::new(ControltowerControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ControltowerControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                control_identifier: self.control_identifier,
                id: core::default::Default::default(),
                target_identifier: self.target_identifier,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ControltowerControlRef {
    shared: StackShared,
    base: String,
}

impl Ref for ControltowerControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ControltowerControlRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `control_identifier` after provisioning.\n"]
    pub fn control_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_identifier` after provisioning.\n"]
    pub fn target_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ControltowerControlTimeoutsElRef {
        ControltowerControlTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ControltowerControlTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ControltowerControlTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for ControltowerControlTimeoutsEl {
    type O = BlockAssignable<ControltowerControlTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildControltowerControlTimeoutsEl {}

impl BuildControltowerControlTimeoutsEl {
    pub fn build(self) -> ControltowerControlTimeoutsEl {
        ControltowerControlTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ControltowerControlTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ControltowerControlTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ControltowerControlTimeoutsElRef {
        ControltowerControlTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ControltowerControlTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
