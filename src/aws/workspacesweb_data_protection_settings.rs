#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspaceswebDataProtectionSettingsData_ {
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
    additional_encryption_context: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_redaction_configuration: std
    ::option
    ::Option<std::vec::Vec<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration>>,
    dynamic: WorkspaceswebDataProtectionSettingsDynamic,
}

struct WorkspaceswebDataProtectionSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkspaceswebDataProtectionSettingsData_>,
}

#[derive(Clone)]
pub struct WorkspaceswebDataProtectionSettings(std::rc::Rc<WorkspaceswebDataProtectionSettings_>);

impl WorkspaceswebDataProtectionSettings {
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

    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `inline_redaction_configuration`.\n"]
    pub fn set_inline_redaction_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inline_redaction_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inline_redaction_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_protection_settings_arn` after provisioning.\n"]
    pub fn data_protection_settings_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_protection_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `inline_redaction_configuration` after provisioning.\n"]
    pub fn inline_redaction_configuration(
        &self,
    ) -> terrars::ListRef<WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.inline_redaction_configuration", self.extract_ref()),
        )
    }
}

impl terrars::Referable for WorkspaceswebDataProtectionSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkspaceswebDataProtectionSettings { }

impl terrars::ToListMappable for WorkspaceswebDataProtectionSettings {
    type O = terrars::ListRef<WorkspaceswebDataProtectionSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkspaceswebDataProtectionSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_workspacesweb_data_protection_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspaceswebDataProtectionSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebDataProtectionSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkspaceswebDataProtectionSettings {
        let out = WorkspaceswebDataProtectionSettings(std::rc::Rc::new(WorkspaceswebDataProtectionSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkspaceswebDataProtectionSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                additional_encryption_context: core::default::Default::default(),
                customer_managed_key: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                inline_redaction_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspaceswebDataProtectionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebDataProtectionSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkspaceswebDataProtectionSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_protection_settings_arn` after provisioning.\n"]
    pub fn data_protection_settings_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_protection_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `inline_redaction_configuration` after provisioning.\n"]
    pub fn inline_redaction_configuration(
        &self,
    ) -> terrars::ListRef<WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.inline_redaction_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebDataProtectionSettingsDynamic {
    inline_redaction_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern_description: std::option::Option<terrars::PrimField<std::string::String>>,
    pattern_name: terrars::PrimField<std::string::String>,
    pattern_regex: terrars::PrimField<std::string::String>,
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
    #[doc = "Set the field `keyword_regex`.\n"]
    pub fn set_keyword_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keyword_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pattern_description`.\n"]
    pub fn set_pattern_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pattern_description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
    type O =
        terrars
        ::BlockAssignable<
            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
    #[doc = ""]
    pub pattern_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub pattern_regex: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
    pub fn build(
        self,
    ) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern {
            keyword_regex: core::default::Default::default(),
            pattern_description: core::default::Default::default(),
            pattern_name: self.pattern_name,
            pattern_regex: self.pattern_regex,
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPatternRef {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keyword_regex` after provisioning.\n"]
    pub fn keyword_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_description` after provisioning.\n"]
    pub fn pattern_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_description", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_name` after provisioning.\n"]
    pub fn pattern_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_name", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_regex` after provisioning.\n"]
    pub fn pattern_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redaction_place_holder_text: std::option::Option<terrars::PrimField<std::string::String>>,
    redaction_place_holder_type: terrars::PrimField<std::string::String>,
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
    #[doc = "Set the field `redaction_place_holder_text`.\n"]
    pub fn set_redaction_place_holder_text(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redaction_place_holder_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
    type O =
        terrars
        ::BlockAssignable<
            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
    #[doc = ""]
    pub redaction_place_holder_type: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
    pub fn build(
        self,
    ) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder {
            redaction_place_holder_text: core::default::Default::default(),
            redaction_place_holder_type: self.redaction_place_holder_type,
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolderRef {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `redaction_place_holder_text` after provisioning.\n"]
    pub fn redaction_place_holder_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redaction_place_holder_text", self.base))
    }

    #[doc = "Get a reference to the value of field `redaction_place_holder_type` after provisioning.\n"]
    pub fn redaction_place_holder_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redaction_place_holder_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternDynamic {
    custom_pattern: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern,
        >,
    >,
    redaction_place_holder: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder,
        >,
    >,
}

#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    built_in_pattern_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforced_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempt_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_pattern: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redaction_place_holder: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder,
        >,
    >,
    dynamic: WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternDynamic,
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {
    #[doc = "Set the field `built_in_pattern_id`.\n"]
    pub fn set_built_in_pattern_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.built_in_pattern_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `confidence_level`.\n"]
    pub fn set_confidence_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.confidence_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforced_urls`.\n"]
    pub fn set_enforced_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enforced_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exempt_urls`.\n"]
    pub fn set_exempt_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exempt_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_pattern`.\n"]
    pub fn set_custom_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPattern,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_pattern = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redaction_place_holder`.\n"]
    pub fn set_redaction_place_holder(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolder,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redaction_place_holder = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redaction_place_holder = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {
    type O =
        terrars
        ::BlockAssignable<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {}

impl BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {
    pub fn build(self) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern {
            built_in_pattern_id: core::default::Default::default(),
            confidence_level: core::default::Default::default(),
            enforced_urls: core::default::Default::default(),
            exempt_urls: core::default::Default::default(),
            custom_pattern: core::default::Default::default(),
            redaction_place_holder: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternRef {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_pattern_id` after provisioning.\n"]
    pub fn built_in_pattern_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.built_in_pattern_id", self.base))
    }

    #[doc = "Get a reference to the value of field `confidence_level` after provisioning.\n"]
    pub fn confidence_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_level", self.base))
    }

    #[doc = "Get a reference to the value of field `enforced_urls` after provisioning.\n"]
    pub fn enforced_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforced_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `exempt_urls` after provisioning.\n"]
    pub fn exempt_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exempt_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_pattern` after provisioning.\n"]
    pub fn custom_pattern(
        &self,
    ) -> terrars
    ::ListRef<
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_CustomPatternRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `redaction_place_holder` after provisioning.\n"]
    pub fn redaction_place_holder(
        &self,
    ) -> terrars
    ::ListRef<
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern_RedactionPlaceHolderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redaction_place_holder", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationDynamic {
    inline_redaction_pattern: std
    ::option
    ::Option<
        terrars::DynamicBlock<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern>,
    >,
}

#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_confidence_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_enforced_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_exempt_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_redaction_pattern: std
    ::option
    ::Option<std::vec::Vec<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern>>,
    dynamic: WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationDynamic,
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {
    #[doc = "Set the field `global_confidence_level`.\n"]
    pub fn set_global_confidence_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.global_confidence_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `global_enforced_urls`.\n"]
    pub fn set_global_enforced_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.global_enforced_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `global_exempt_urls`.\n"]
    pub fn set_global_exempt_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.global_exempt_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inline_redaction_pattern`.\n"]
    pub fn set_inline_redaction_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPattern,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_redaction_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_redaction_pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {
    type O = terrars::BlockAssignable<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {}

impl BuildWorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {
    pub fn build(self) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration {
            global_confidence_level: core::default::Default::default(),
            global_enforced_urls: core::default::Default::default(),
            global_exempt_urls: core::default::Default::default(),
            inline_redaction_pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef {
        WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebDataProtectionSettings_InlineRedactionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `global_confidence_level` after provisioning.\n"]
    pub fn global_confidence_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_confidence_level", self.base))
    }

    #[doc = "Get a reference to the value of field `global_enforced_urls` after provisioning.\n"]
    pub fn global_enforced_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_enforced_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `global_exempt_urls` after provisioning.\n"]
    pub fn global_exempt_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_exempt_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `inline_redaction_pattern` after provisioning.\n"]
    pub fn inline_redaction_pattern(
        &self,
    ) -> terrars::ListRef<WorkspaceswebDataProtectionSettings_InlineRedactionConfiguration_InlineRedactionPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_redaction_pattern", self.base))
    }
}
