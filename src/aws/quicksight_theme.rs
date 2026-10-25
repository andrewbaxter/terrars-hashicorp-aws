#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightThemeData_ {
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
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    base_theme_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    theme_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<std::vec::Vec<QuicksightTheme_Permissions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<QuicksightTheme_Timeouts>,
    dynamic: QuicksightThemeDynamic,
}

struct QuicksightTheme_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightThemeData_>,
}

#[derive(Clone)]
pub struct QuicksightTheme(std::rc::Rc<QuicksightTheme_>);

impl QuicksightTheme {
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

    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_description`.\n"]
    pub fn set_version_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Permissions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<QuicksightTheme_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_theme_id` after provisioning.\n"]
    pub fn base_theme_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_theme_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.theme_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<QuicksightTheme_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightTheme_TimeoutsRef {
        QuicksightTheme_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for QuicksightTheme {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QuicksightTheme { }

impl terrars::ToListMappable for QuicksightTheme {
    type O = terrars::ListRef<QuicksightThemeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QuicksightTheme_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_quicksight_theme".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQuicksightTheme {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub base_theme_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub theme_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightTheme {
    pub fn build(self, stack: &mut terrars::Stack) -> QuicksightTheme {
        let out = QuicksightTheme(std::rc::Rc::new(QuicksightTheme_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QuicksightThemeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                base_theme_id: self.base_theme_id,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                theme_id: self.theme_id,
                version_description: core::default::Default::default(),
                configuration: core::default::Default::default(),
                permissions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QuicksightThemeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightThemeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QuicksightThemeRef {
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

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_theme_id` after provisioning.\n"]
    pub fn base_theme_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_theme_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.theme_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<QuicksightTheme_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightTheme_TimeoutsRef {
        QuicksightTheme_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct QuicksightThemeDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration>>,
    permissions: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Permissions>>,
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_DataColorPalette {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    colors: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_fill_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_max_gradient: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl QuicksightTheme_Configuration_DataColorPalette {
    #[doc = "Set the field `colors`.\n"]
    pub fn set_colors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.colors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `empty_fill_color`.\n"]
    pub fn set_empty_fill_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.empty_fill_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_max_gradient`.\n"]
    pub fn set_min_max_gradient(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.min_max_gradient = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_DataColorPalette {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_DataColorPalette>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_DataColorPalette {}

impl BuildQuicksightTheme_Configuration_DataColorPalette {
    pub fn build(self) -> QuicksightTheme_Configuration_DataColorPalette {
        QuicksightTheme_Configuration_DataColorPalette {
            colors: core::default::Default::default(),
            empty_fill_color: core::default::Default::default(),
            min_max_gradient: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_DataColorPaletteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_DataColorPaletteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_DataColorPaletteRef {
        QuicksightTheme_Configuration_DataColorPaletteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_DataColorPaletteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `colors` after provisioning.\n"]
    pub fn colors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.colors", self.base))
    }

    #[doc = "Get a reference to the value of field `empty_fill_color` after provisioning.\n"]
    pub fn empty_fill_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.empty_fill_color", self.base))
    }

    #[doc = "Get a reference to the value of field `min_max_gradient` after provisioning.\n"]
    pub fn min_max_gradient(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.min_max_gradient", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Sheet_Tile_Border {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl QuicksightTheme_Configuration_Sheet_Tile_Border {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Sheet_Tile_Border {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_Tile_Border>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Sheet_Tile_Border {}

impl BuildQuicksightTheme_Configuration_Sheet_Tile_Border {
    pub fn build(self) -> QuicksightTheme_Configuration_Sheet_Tile_Border {
        QuicksightTheme_Configuration_Sheet_Tile_Border { show: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_Sheet_Tile_BorderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_Sheet_Tile_BorderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_Sheet_Tile_BorderRef {
        QuicksightTheme_Configuration_Sheet_Tile_BorderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_Sheet_Tile_BorderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightTheme_Configuration_Sheet_TileDynamic {
    border: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Sheet_Tile_Border>>,
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Sheet_Tile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    border: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Sheet_Tile_Border>>,
    dynamic: QuicksightTheme_Configuration_Sheet_TileDynamic,
}

impl QuicksightTheme_Configuration_Sheet_Tile {
    #[doc = "Set the field `border`.\n"]
    pub fn set_border(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_Tile_Border>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.border = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.border = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Sheet_Tile {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_Tile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Sheet_Tile {}

impl BuildQuicksightTheme_Configuration_Sheet_Tile {
    pub fn build(self) -> QuicksightTheme_Configuration_Sheet_Tile {
        QuicksightTheme_Configuration_Sheet_Tile {
            border: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_Sheet_TileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_Sheet_TileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightTheme_Configuration_Sheet_TileRef {
        QuicksightTheme_Configuration_Sheet_TileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_Sheet_TileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `border` after provisioning.\n"]
    pub fn border(&self) -> terrars::ListRef<QuicksightTheme_Configuration_Sheet_Tile_BorderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.border", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl QuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_TileLayout_Gutter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {}

impl BuildQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    pub fn build(self) -> QuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
        QuicksightTheme_Configuration_Sheet_TileLayout_Gutter { show: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
        QuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl QuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_TileLayout_Margin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Sheet_TileLayout_Margin {}

impl BuildQuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    pub fn build(self) -> QuicksightTheme_Configuration_Sheet_TileLayout_Margin {
        QuicksightTheme_Configuration_Sheet_TileLayout_Margin { show: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
        QuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightTheme_Configuration_Sheet_TileLayoutDynamic {
    gutter: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Sheet_TileLayout_Gutter>>,
    margin: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Sheet_TileLayout_Margin>>,
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Sheet_TileLayout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gutter: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Sheet_TileLayout_Gutter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    margin: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Sheet_TileLayout_Margin>>,
    dynamic: QuicksightTheme_Configuration_Sheet_TileLayoutDynamic,
}

impl QuicksightTheme_Configuration_Sheet_TileLayout {
    #[doc = "Set the field `gutter`.\n"]
    pub fn set_gutter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_TileLayout_Gutter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gutter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gutter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `margin`.\n"]
    pub fn set_margin(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_TileLayout_Margin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.margin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.margin = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Sheet_TileLayout {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_TileLayout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Sheet_TileLayout {}

impl BuildQuicksightTheme_Configuration_Sheet_TileLayout {
    pub fn build(self) -> QuicksightTheme_Configuration_Sheet_TileLayout {
        QuicksightTheme_Configuration_Sheet_TileLayout {
            gutter: core::default::Default::default(),
            margin: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_Sheet_TileLayoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_Sheet_TileLayoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_Sheet_TileLayoutRef {
        QuicksightTheme_Configuration_Sheet_TileLayoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_Sheet_TileLayoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gutter` after provisioning.\n"]
    pub fn gutter(&self) -> terrars::ListRef<QuicksightTheme_Configuration_Sheet_TileLayout_GutterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gutter", self.base))
    }

    #[doc = "Get a reference to the value of field `margin` after provisioning.\n"]
    pub fn margin(&self) -> terrars::ListRef<QuicksightTheme_Configuration_Sheet_TileLayout_MarginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.margin", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightTheme_Configuration_SheetDynamic {
    tile: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Sheet_Tile>>,
    tile_layout: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Sheet_TileLayout>>,
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Sheet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Sheet_Tile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile_layout: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Sheet_TileLayout>>,
    dynamic: QuicksightTheme_Configuration_SheetDynamic,
}

impl QuicksightTheme_Configuration_Sheet {
    #[doc = "Set the field `tile`.\n"]
    pub fn set_tile(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_Tile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tile_layout`.\n"]
    pub fn set_tile_layout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet_TileLayout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tile_layout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tile_layout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Sheet {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Sheet {}

impl BuildQuicksightTheme_Configuration_Sheet {
    pub fn build(self) -> QuicksightTheme_Configuration_Sheet {
        QuicksightTheme_Configuration_Sheet {
            tile: core::default::Default::default(),
            tile_layout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_SheetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_SheetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightTheme_Configuration_SheetRef {
        QuicksightTheme_Configuration_SheetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_SheetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tile` after provisioning.\n"]
    pub fn tile(&self) -> terrars::ListRef<QuicksightTheme_Configuration_Sheet_TileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tile", self.base))
    }

    #[doc = "Get a reference to the value of field `tile_layout` after provisioning.\n"]
    pub fn tile_layout(&self) -> terrars::ListRef<QuicksightTheme_Configuration_Sheet_TileLayoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tile_layout", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Typography_FontFamilies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightTheme_Configuration_Typography_FontFamilies {
    #[doc = "Set the field `font_family`.\n"]
    pub fn set_font_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Typography_FontFamilies {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Typography_FontFamilies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Typography_FontFamilies {}

impl BuildQuicksightTheme_Configuration_Typography_FontFamilies {
    pub fn build(self) -> QuicksightTheme_Configuration_Typography_FontFamilies {
        QuicksightTheme_Configuration_Typography_FontFamilies { font_family: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_Typography_FontFamiliesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_Typography_FontFamiliesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_Typography_FontFamiliesRef {
        QuicksightTheme_Configuration_Typography_FontFamiliesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_Typography_FontFamiliesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `font_family` after provisioning.\n"]
    pub fn font_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_family", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightTheme_Configuration_TypographyDynamic {
    font_families: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Typography_FontFamilies>>,
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_Typography {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_families: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Typography_FontFamilies>>,
    dynamic: QuicksightTheme_Configuration_TypographyDynamic,
}

impl QuicksightTheme_Configuration_Typography {
    #[doc = "Set the field `font_families`.\n"]
    pub fn set_font_families(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Typography_FontFamilies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.font_families = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.font_families = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_Typography {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_Typography>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_Typography {}

impl BuildQuicksightTheme_Configuration_Typography {
    pub fn build(self) -> QuicksightTheme_Configuration_Typography {
        QuicksightTheme_Configuration_Typography {
            font_families: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_TypographyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_TypographyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightTheme_Configuration_TypographyRef {
        QuicksightTheme_Configuration_TypographyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_TypographyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `font_families` after provisioning.\n"]
    pub fn font_families(&self) -> terrars::ListRef<QuicksightTheme_Configuration_Typography_FontFamiliesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.font_families", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration_UiColorPalette {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accent_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    danger: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    danger_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    measure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    measure_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_background: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_background: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warning: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warning_foreground: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightTheme_Configuration_UiColorPalette {
    #[doc = "Set the field `accent`.\n"]
    pub fn set_accent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accent_foreground`.\n"]
    pub fn set_accent_foreground(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accent_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `danger`.\n"]
    pub fn set_danger(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.danger = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `danger_foreground`.\n"]
    pub fn set_danger_foreground(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.danger_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension_foreground`.\n"]
    pub fn set_dimension_foreground(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dimension_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `measure`.\n"]
    pub fn set_measure(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.measure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `measure_foreground`.\n"]
    pub fn set_measure_foreground(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.measure_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_background`.\n"]
    pub fn set_primary_background(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.primary_background = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_foreground`.\n"]
    pub fn set_primary_foreground(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.primary_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_background`.\n"]
    pub fn set_secondary_background(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_background = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_foreground`.\n"]
    pub fn set_secondary_foreground(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success`.\n"]
    pub fn set_success(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.success = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_foreground`.\n"]
    pub fn set_success_foreground(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.success_foreground = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warning`.\n"]
    pub fn set_warning(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.warning = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warning_foreground`.\n"]
    pub fn set_warning_foreground(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.warning_foreground = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration_UiColorPalette {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration_UiColorPalette>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration_UiColorPalette {}

impl BuildQuicksightTheme_Configuration_UiColorPalette {
    pub fn build(self) -> QuicksightTheme_Configuration_UiColorPalette {
        QuicksightTheme_Configuration_UiColorPalette {
            accent: core::default::Default::default(),
            accent_foreground: core::default::Default::default(),
            danger: core::default::Default::default(),
            danger_foreground: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dimension_foreground: core::default::Default::default(),
            measure: core::default::Default::default(),
            measure_foreground: core::default::Default::default(),
            primary_background: core::default::Default::default(),
            primary_foreground: core::default::Default::default(),
            secondary_background: core::default::Default::default(),
            secondary_foreground: core::default::Default::default(),
            success: core::default::Default::default(),
            success_foreground: core::default::Default::default(),
            warning: core::default::Default::default(),
            warning_foreground: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_Configuration_UiColorPaletteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_Configuration_UiColorPaletteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightTheme_Configuration_UiColorPaletteRef {
        QuicksightTheme_Configuration_UiColorPaletteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_Configuration_UiColorPaletteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accent` after provisioning.\n"]
    pub fn accent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accent", self.base))
    }

    #[doc = "Get a reference to the value of field `accent_foreground` after provisioning.\n"]
    pub fn accent_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accent_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `danger` after provisioning.\n"]
    pub fn danger(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.danger", self.base))
    }

    #[doc = "Get a reference to the value of field `danger_foreground` after provisioning.\n"]
    pub fn danger_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.danger_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension_foreground` after provisioning.\n"]
    pub fn dimension_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `measure` after provisioning.\n"]
    pub fn measure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.measure", self.base))
    }

    #[doc = "Get a reference to the value of field `measure_foreground` after provisioning.\n"]
    pub fn measure_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.measure_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_background` after provisioning.\n"]
    pub fn primary_background(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_background", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_foreground` after provisioning.\n"]
    pub fn primary_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_background` after provisioning.\n"]
    pub fn secondary_background(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_background", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_foreground` after provisioning.\n"]
    pub fn secondary_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success", self.base))
    }

    #[doc = "Get a reference to the value of field `success_foreground` after provisioning.\n"]
    pub fn success_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_foreground", self.base))
    }

    #[doc = "Get a reference to the value of field `warning` after provisioning.\n"]
    pub fn warning(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning", self.base))
    }

    #[doc = "Get a reference to the value of field `warning_foreground` after provisioning.\n"]
    pub fn warning_foreground(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_foreground", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightTheme_ConfigurationDynamic {
    data_color_palette: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_DataColorPalette>>,
    sheet: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Sheet>>,
    typography: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_Typography>>,
    ui_color_palette: std::option::Option<terrars::DynamicBlock<QuicksightTheme_Configuration_UiColorPalette>>,
}

#[derive(Serialize)]
pub struct QuicksightTheme_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_color_palette: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_DataColorPalette>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sheet: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Sheet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    typography: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_Typography>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_color_palette: std::option::Option<std::vec::Vec<QuicksightTheme_Configuration_UiColorPalette>>,
    dynamic: QuicksightTheme_ConfigurationDynamic,
}

impl QuicksightTheme_Configuration {
    #[doc = "Set the field `data_color_palette`.\n"]
    pub fn set_data_color_palette(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_DataColorPalette>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_color_palette = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_color_palette = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sheet`.\n"]
    pub fn set_sheet(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Sheet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sheet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sheet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `typography`.\n"]
    pub fn set_typography(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_Typography>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.typography = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.typography = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ui_color_palette`.\n"]
    pub fn set_ui_color_palette(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightTheme_Configuration_UiColorPalette>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ui_color_palette = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ui_color_palette = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Configuration {
    type O = terrars::BlockAssignable<QuicksightTheme_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Configuration {}

impl BuildQuicksightTheme_Configuration {
    pub fn build(self) -> QuicksightTheme_Configuration {
        QuicksightTheme_Configuration {
            data_color_palette: core::default::Default::default(),
            sheet: core::default::Default::default(),
            typography: core::default::Default::default(),
            ui_color_palette: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightTheme_ConfigurationRef {
        QuicksightTheme_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_color_palette` after provisioning.\n"]
    pub fn data_color_palette(&self) -> terrars::ListRef<QuicksightTheme_Configuration_DataColorPaletteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_color_palette", self.base))
    }

    #[doc = "Get a reference to the value of field `sheet` after provisioning.\n"]
    pub fn sheet(&self) -> terrars::ListRef<QuicksightTheme_Configuration_SheetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sheet", self.base))
    }

    #[doc = "Get a reference to the value of field `typography` after provisioning.\n"]
    pub fn typography(&self) -> terrars::ListRef<QuicksightTheme_Configuration_TypographyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.typography", self.base))
    }

    #[doc = "Get a reference to the value of field `ui_color_palette` after provisioning.\n"]
    pub fn ui_color_palette(&self) -> terrars::ListRef<QuicksightTheme_Configuration_UiColorPaletteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_color_palette", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Permissions {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
}

impl QuicksightTheme_Permissions { }

impl terrars::ToListMappable for QuicksightTheme_Permissions {
    type O = terrars::BlockAssignable<QuicksightTheme_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Permissions {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightTheme_Permissions {
    pub fn build(self) -> QuicksightTheme_Permissions {
        QuicksightTheme_Permissions {
            actions: self.actions,
            principal: self.principal,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightTheme_PermissionsRef {
        QuicksightTheme_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_PermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightTheme_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightTheme_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightTheme_Timeouts {
    type O = terrars::BlockAssignable<QuicksightTheme_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightTheme_Timeouts {}

impl BuildQuicksightTheme_Timeouts {
    pub fn build(self) -> QuicksightTheme_Timeouts {
        QuicksightTheme_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightTheme_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightTheme_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightTheme_TimeoutsRef {
        QuicksightTheme_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightTheme_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
