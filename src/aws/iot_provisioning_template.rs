use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotProvisioningTemplateData {
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
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    provisioning_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    template_body: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_provisioning_hook: Option<Vec<IotProvisioningTemplatePreProvisioningHookEl>>,
    dynamic: IotProvisioningTemplateDynamic,
}

struct IotProvisioningTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IotProvisioningTemplateData>,
}

#[derive(Clone)]
pub struct IotProvisioningTemplate(Rc<IotProvisioningTemplate_>);

impl IotProvisioningTemplate {
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

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
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

    #[doc= "Set the field `pre_provisioning_hook`.\n"]
    pub fn set_pre_provisioning_hook(
        self,
        v: impl Into<BlockAssignable<IotProvisioningTemplatePreProvisioningHookEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pre_provisioning_hook = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pre_provisioning_hook = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_version_id` after provisioning.\n"]
    pub fn default_version_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_version_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provisioning_role_arn` after provisioning.\n"]
    pub fn provisioning_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioning_role_arn", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `pre_provisioning_hook` after provisioning.\n"]
    pub fn pre_provisioning_hook(&self) -> ListRef<IotProvisioningTemplatePreProvisioningHookElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pre_provisioning_hook", self.extract_ref()))
    }
}

impl Resource for IotProvisioningTemplate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for IotProvisioningTemplate {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for IotProvisioningTemplate {
    type O = ListRef<IotProvisioningTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for IotProvisioningTemplate_ {
    fn extract_resource_type(&self) -> String {
        "aws_iot_provisioning_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotProvisioningTemplate {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub provisioning_role_arn: PrimField<String>,
    #[doc= ""]
    pub template_body: PrimField<String>,
}

impl BuildIotProvisioningTemplate {
    pub fn build(self, stack: &mut Stack) -> IotProvisioningTemplate {
        let out = IotProvisioningTemplate(Rc::new(IotProvisioningTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IotProvisioningTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                provisioning_role_arn: self.provisioning_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                template_body: self.template_body,
                pre_provisioning_hook: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotProvisioningTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotProvisioningTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IotProvisioningTemplateRef {
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

    #[doc= "Get a reference to the value of field `default_version_id` after provisioning.\n"]
    pub fn default_version_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_version_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provisioning_role_arn` after provisioning.\n"]
    pub fn provisioning_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioning_role_arn", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `pre_provisioning_hook` after provisioning.\n"]
    pub fn pre_provisioning_hook(&self) -> ListRef<IotProvisioningTemplatePreProvisioningHookElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pre_provisioning_hook", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IotProvisioningTemplatePreProvisioningHookEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_version: Option<PrimField<String>>,
    target_arn: PrimField<String>,
}

impl IotProvisioningTemplatePreProvisioningHookEl {
    #[doc= "Set the field `payload_version`.\n"]
    pub fn set_payload_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload_version = Some(v.into());
        self
    }
}

impl ToListMappable for IotProvisioningTemplatePreProvisioningHookEl {
    type O = BlockAssignable<IotProvisioningTemplatePreProvisioningHookEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotProvisioningTemplatePreProvisioningHookEl {
    #[doc= ""]
    pub target_arn: PrimField<String>,
}

impl BuildIotProvisioningTemplatePreProvisioningHookEl {
    pub fn build(self) -> IotProvisioningTemplatePreProvisioningHookEl {
        IotProvisioningTemplatePreProvisioningHookEl {
            payload_version: core::default::Default::default(),
            target_arn: self.target_arn,
        }
    }
}

pub struct IotProvisioningTemplatePreProvisioningHookElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotProvisioningTemplatePreProvisioningHookElRef {
    fn new(shared: StackShared, base: String) -> IotProvisioningTemplatePreProvisioningHookElRef {
        IotProvisioningTemplatePreProvisioningHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotProvisioningTemplatePreProvisioningHookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `payload_version` after provisioning.\n"]
    pub fn payload_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload_version", self.base))
    }

    #[doc= "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotProvisioningTemplateDynamic {
    pre_provisioning_hook: Option<DynamicBlock<IotProvisioningTemplatePreProvisioningHookEl>>,
}
