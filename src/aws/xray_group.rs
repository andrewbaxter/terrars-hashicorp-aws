use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct XrayGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    filter_expression: PrimField<String>,
    group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insights_configuration: Option<Vec<XrayGroupInsightsConfigurationEl>>,
    dynamic: XrayGroupDynamic,
}

struct XrayGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<XrayGroupData>,
}

#[derive(Clone)]
pub struct XrayGroup(Rc<XrayGroup_>);

impl XrayGroup {
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

    #[doc= "Set the field `insights_configuration`.\n"]
    pub fn set_insights_configuration(self, v: impl Into<BlockAssignable<XrayGroupInsightsConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().insights_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.insights_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter_expression` after provisioning.\n"]
    pub fn filter_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_expression", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `insights_configuration` after provisioning.\n"]
    pub fn insights_configuration(&self) -> ListRef<XrayGroupInsightsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.insights_configuration", self.extract_ref()))
    }
}

impl Resource for XrayGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for XrayGroup {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for XrayGroup {
    type O = ListRef<XrayGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for XrayGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_xray_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildXrayGroup {
    pub tf_id: String,
    #[doc= ""]
    pub filter_expression: PrimField<String>,
    #[doc= ""]
    pub group_name: PrimField<String>,
}

impl BuildXrayGroup {
    pub fn build(self, stack: &mut Stack) -> XrayGroup {
        let out = XrayGroup(Rc::new(XrayGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(XrayGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                filter_expression: self.filter_expression,
                group_name: self.group_name,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                insights_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct XrayGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for XrayGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl XrayGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter_expression` after provisioning.\n"]
    pub fn filter_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_expression", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `insights_configuration` after provisioning.\n"]
    pub fn insights_configuration(&self) -> ListRef<XrayGroupInsightsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.insights_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct XrayGroupInsightsConfigurationEl {
    insights_enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notifications_enabled: Option<PrimField<bool>>,
}

impl XrayGroupInsightsConfigurationEl {
    #[doc= "Set the field `notifications_enabled`.\n"]
    pub fn set_notifications_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.notifications_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for XrayGroupInsightsConfigurationEl {
    type O = BlockAssignable<XrayGroupInsightsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildXrayGroupInsightsConfigurationEl {
    #[doc= ""]
    pub insights_enabled: PrimField<bool>,
}

impl BuildXrayGroupInsightsConfigurationEl {
    pub fn build(self) -> XrayGroupInsightsConfigurationEl {
        XrayGroupInsightsConfigurationEl {
            insights_enabled: self.insights_enabled,
            notifications_enabled: core::default::Default::default(),
        }
    }
}

pub struct XrayGroupInsightsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for XrayGroupInsightsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> XrayGroupInsightsConfigurationElRef {
        XrayGroupInsightsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl XrayGroupInsightsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `insights_enabled` after provisioning.\n"]
    pub fn insights_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insights_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `notifications_enabled` after provisioning.\n"]
    pub fn notifications_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notifications_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct XrayGroupDynamic {
    insights_configuration: Option<DynamicBlock<XrayGroupInsightsConfigurationEl>>,
}
