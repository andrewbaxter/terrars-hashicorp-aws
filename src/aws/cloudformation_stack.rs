use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudformationStackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_rollback: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudformationStackTimeoutsEl>,
}

struct CloudformationStack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudformationStackData>,
}

#[derive(Clone)]
pub struct CloudformationStack(Rc<CloudformationStack_>);

impl CloudformationStack {
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

    #[doc= "Set the field `capabilities`.\n"]
    pub fn set_capabilities(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().capabilities = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_rollback`.\n"]
    pub fn set_disable_rollback(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_rollback = Some(v.into());
        self
    }

    #[doc= "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `notification_arns`.\n"]
    pub fn set_notification_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().notification_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `on_failure`.\n"]
    pub fn set_on_failure(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_failure = Some(v.into());
        self
    }

    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }

    #[doc= "Set the field `policy_body`.\n"]
    pub fn set_policy_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_body = Some(v.into());
        self
    }

    #[doc= "Set the field `policy_url`.\n"]
    pub fn set_policy_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_url = Some(v.into());
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

    #[doc= "Set the field `template_body`.\n"]
    pub fn set_template_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_body = Some(v.into());
        self
    }

    #[doc= "Set the field `template_url`.\n"]
    pub fn set_template_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_url = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_in_minutes = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudformationStackTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_rollback` after provisioning.\n"]
    pub fn disable_rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_rollback", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_arns` after provisioning.\n"]
    pub fn notification_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.notification_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_failure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.outputs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_body` after provisioning.\n"]
    pub fn policy_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_url` after provisioning.\n"]
    pub fn policy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_minutes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackTimeoutsElRef {
        CloudformationStackTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for CloudformationStack {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudformationStack { }

impl ToListMappable for CloudformationStack {
    type O = ListRef<CloudformationStackRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudformationStack_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudformation_stack".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudformationStack {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildCloudformationStack {
    pub fn build(self, stack: &mut Stack) -> CloudformationStack {
        let out = CloudformationStack(Rc::new(CloudformationStack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudformationStackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capabilities: core::default::Default::default(),
                disable_rollback: core::default::Default::default(),
                iam_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                notification_arns: core::default::Default::default(),
                on_failure: core::default::Default::default(),
                parameters: core::default::Default::default(),
                policy_body: core::default::Default::default(),
                policy_url: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                template_body: core::default::Default::default(),
                template_url: core::default::Default::default(),
                timeout_in_minutes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudformationStackRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudformationStackRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_rollback` after provisioning.\n"]
    pub fn disable_rollback(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_rollback", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_arns` after provisioning.\n"]
    pub fn notification_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.notification_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_failure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.outputs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_body` after provisioning.\n"]
    pub fn policy_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_url` after provisioning.\n"]
    pub fn policy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_minutes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackTimeoutsElRef {
        CloudformationStackTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CloudformationStackTimeoutsEl {
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

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for CloudformationStackTimeoutsEl {
    type O = BlockAssignable<CloudformationStackTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackTimeoutsEl {}

impl BuildCloudformationStackTimeoutsEl {
    pub fn build(self) -> CloudformationStackTimeoutsEl {
        CloudformationStackTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudformationStackTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudformationStackTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudformationStackTimeoutsElRef {
        CloudformationStackTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackTimeoutsElRef {
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

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
