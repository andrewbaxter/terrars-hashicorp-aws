#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PinpointEmailTemplateData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    template_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_template: std::option::Option<std::vec::Vec<PinpointEmailTemplate_EmailTemplate>>,
    dynamic: PinpointEmailTemplateDynamic,
}

struct PinpointEmailTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PinpointEmailTemplateData_>,
}

#[derive(Clone)]
pub struct PinpointEmailTemplate(std::rc::Rc<PinpointEmailTemplate_>);

impl PinpointEmailTemplate {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_template`.\n"]
    pub fn set_email_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PinpointEmailTemplate_EmailTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_template` after provisioning.\n"]
    pub fn email_template(&self) -> terrars::ListRef<PinpointEmailTemplate_EmailTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_template", self.extract_ref()))
    }
}

impl terrars::Referable for PinpointEmailTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PinpointEmailTemplate { }

impl terrars::ToListMappable for PinpointEmailTemplate {
    type O = terrars::ListRef<PinpointEmailTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PinpointEmailTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_pinpoint_email_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPinpointEmailTemplate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub template_name: terrars::PrimField<std::string::String>,
}

impl BuildPinpointEmailTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> PinpointEmailTemplate {
        let out = PinpointEmailTemplate(std::rc::Rc::new(PinpointEmailTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PinpointEmailTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                template_name: self.template_name,
                email_template: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PinpointEmailTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointEmailTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PinpointEmailTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_template` after provisioning.\n"]
    pub fn email_template(&self) -> terrars::ListRef<PinpointEmailTemplate_EmailTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_template", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PinpointEmailTemplateDynamic {
    email_template: std::option::Option<terrars::DynamicBlock<PinpointEmailTemplate_EmailTemplate>>,
}

#[derive(Serialize)]
pub struct PinpointEmailTemplate_EmailTemplate_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PinpointEmailTemplate_EmailTemplate_Header {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PinpointEmailTemplate_EmailTemplate_Header {
    type O = terrars::BlockAssignable<PinpointEmailTemplate_EmailTemplate_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointEmailTemplate_EmailTemplate_Header {}

impl BuildPinpointEmailTemplate_EmailTemplate_Header {
    pub fn build(self) -> PinpointEmailTemplate_EmailTemplate_Header {
        PinpointEmailTemplate_EmailTemplate_Header {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PinpointEmailTemplate_EmailTemplate_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointEmailTemplate_EmailTemplate_HeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PinpointEmailTemplate_EmailTemplate_HeaderRef {
        PinpointEmailTemplate_EmailTemplate_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointEmailTemplate_EmailTemplate_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct PinpointEmailTemplate_EmailTemplateDynamic {
    header: std::option::Option<terrars::DynamicBlock<PinpointEmailTemplate_EmailTemplate_Header>>,
}

#[derive(Serialize)]
pub struct PinpointEmailTemplate_EmailTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_substitutions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html_part: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recommender_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_part: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<PinpointEmailTemplate_EmailTemplate_Header>>,
    dynamic: PinpointEmailTemplate_EmailTemplateDynamic,
}

impl PinpointEmailTemplate_EmailTemplate {
    #[doc = "Set the field `default_substitutions`.\n"]
    pub fn set_default_substitutions(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_substitutions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html_part`.\n"]
    pub fn set_html_part(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.html_part = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recommender_id`.\n"]
    pub fn set_recommender_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recommender_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_part`.\n"]
    pub fn set_text_part(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_part = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PinpointEmailTemplate_EmailTemplate_Header>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PinpointEmailTemplate_EmailTemplate {
    type O = terrars::BlockAssignable<PinpointEmailTemplate_EmailTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointEmailTemplate_EmailTemplate {}

impl BuildPinpointEmailTemplate_EmailTemplate {
    pub fn build(self) -> PinpointEmailTemplate_EmailTemplate {
        PinpointEmailTemplate_EmailTemplate {
            default_substitutions: core::default::Default::default(),
            description: core::default::Default::default(),
            html_part: core::default::Default::default(),
            recommender_id: core::default::Default::default(),
            subject: core::default::Default::default(),
            text_part: core::default::Default::default(),
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PinpointEmailTemplate_EmailTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointEmailTemplate_EmailTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PinpointEmailTemplate_EmailTemplateRef {
        PinpointEmailTemplate_EmailTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointEmailTemplate_EmailTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_substitutions` after provisioning.\n"]
    pub fn default_substitutions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_substitutions", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `html_part` after provisioning.\n"]
    pub fn html_part(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html_part", self.base))
    }

    #[doc = "Get a reference to the value of field `recommender_id` after provisioning.\n"]
    pub fn recommender_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recommender_id", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `text_part` after provisioning.\n"]
    pub fn text_part(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_part", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<PinpointEmailTemplate_EmailTemplate_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
