#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataQuicksightThemeData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    theme_id: terrars::PrimField<std::string::String>,
}

struct DataQuicksightTheme_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataQuicksightThemeData_>,
}

#[derive(Clone)]
pub struct DataQuicksightTheme(std::rc::Rc<DataQuicksightTheme_>);

impl DataQuicksightTheme {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataQuicksightTheme_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<DataQuicksightTheme_PermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
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
}

impl terrars::Referable for DataQuicksightTheme {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataQuicksightTheme { }

impl terrars::ToListMappable for DataQuicksightTheme {
    type O = terrars::ListRef<DataQuicksightThemeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataQuicksightTheme_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_quicksight_theme".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataQuicksightTheme {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub theme_id: terrars::PrimField<std::string::String>,
}

impl BuildDataQuicksightTheme {
    pub fn build(self, stack: &mut terrars::Stack) -> DataQuicksightTheme {
        let out = DataQuicksightTheme(std::rc::Rc::new(DataQuicksightTheme_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataQuicksightThemeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                aws_account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                theme_id: self.theme_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataQuicksightThemeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightThemeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataQuicksightThemeRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataQuicksightTheme_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<DataQuicksightTheme_PermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration_DataColorPalette {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    colors: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_fill_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_max_gradient: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataQuicksightTheme_Configuration_DataColorPalette {
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

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_DataColorPalette {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_DataColorPalette>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_DataColorPalette {}

impl BuildDataQuicksightTheme_Configuration_DataColorPalette {
    pub fn build(self) -> DataQuicksightTheme_Configuration_DataColorPalette {
        DataQuicksightTheme_Configuration_DataColorPalette {
            colors: core::default::Default::default(),
            empty_fill_color: core::default::Default::default(),
            min_max_gradient: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_DataColorPaletteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_DataColorPaletteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_DataColorPaletteRef {
        DataQuicksightTheme_Configuration_DataColorPaletteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_DataColorPaletteRef {
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
pub struct DataQuicksightTheme_Configuration_Sheet_Tile_Border {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl DataQuicksightTheme_Configuration_Sheet_Tile_Border {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Sheet_Tile_Border {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Sheet_Tile_Border>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Sheet_Tile_Border {}

impl BuildDataQuicksightTheme_Configuration_Sheet_Tile_Border {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Sheet_Tile_Border {
        DataQuicksightTheme_Configuration_Sheet_Tile_Border { show: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_Sheet_Tile_BorderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_Sheet_Tile_BorderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_Sheet_Tile_BorderRef {
        DataQuicksightTheme_Configuration_Sheet_Tile_BorderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_Sheet_Tile_BorderRef {
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
pub struct DataQuicksightTheme_Configuration_Sheet_Tile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    border: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_Tile_Border>>,
}

impl DataQuicksightTheme_Configuration_Sheet_Tile {
    #[doc = "Set the field `border`.\n"]
    pub fn set_border(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_Tile_Border>>,
    ) -> Self {
        self.border = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Sheet_Tile {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Sheet_Tile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Sheet_Tile {}

impl BuildDataQuicksightTheme_Configuration_Sheet_Tile {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Sheet_Tile {
        DataQuicksightTheme_Configuration_Sheet_Tile { border: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_Sheet_TileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_Sheet_TileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_Sheet_TileRef {
        DataQuicksightTheme_Configuration_Sheet_TileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_Sheet_TileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `border` after provisioning.\n"]
    pub fn border(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_Sheet_Tile_BorderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.border", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {}

impl BuildDataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter {
        DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter { show: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
        DataQuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_Sheet_TileLayout_GutterRef {
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
pub struct DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Sheet_TileLayout_Margin {}

impl BuildDataQuicksightTheme_Configuration_Sheet_TileLayout_Margin {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin {
        DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin { show: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
        DataQuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_Sheet_TileLayout_MarginRef {
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
pub struct DataQuicksightTheme_Configuration_Sheet_TileLayout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gutter: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    margin: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin>>,
}

impl DataQuicksightTheme_Configuration_Sheet_TileLayout {
    #[doc = "Set the field `gutter`.\n"]
    pub fn set_gutter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_TileLayout_Gutter>>,
    ) -> Self {
        self.gutter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `margin`.\n"]
    pub fn set_margin(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_TileLayout_Margin>>,
    ) -> Self {
        self.margin = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Sheet_TileLayout {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Sheet_TileLayout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Sheet_TileLayout {}

impl BuildDataQuicksightTheme_Configuration_Sheet_TileLayout {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Sheet_TileLayout {
        DataQuicksightTheme_Configuration_Sheet_TileLayout {
            gutter: core::default::Default::default(),
            margin: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_Sheet_TileLayoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_Sheet_TileLayoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_Sheet_TileLayoutRef {
        DataQuicksightTheme_Configuration_Sheet_TileLayoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_Sheet_TileLayoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gutter` after provisioning.\n"]
    pub fn gutter(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_Sheet_TileLayout_GutterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gutter", self.base))
    }

    #[doc = "Get a reference to the value of field `margin` after provisioning.\n"]
    pub fn margin(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_Sheet_TileLayout_MarginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.margin", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration_Sheet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_Tile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile_layout: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_TileLayout>>,
}

impl DataQuicksightTheme_Configuration_Sheet {
    #[doc = "Set the field `tile`.\n"]
    pub fn set_tile(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_Tile>>,
    ) -> Self {
        self.tile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tile_layout`.\n"]
    pub fn set_tile_layout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Sheet_TileLayout>>,
    ) -> Self {
        self.tile_layout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Sheet {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Sheet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Sheet {}

impl BuildDataQuicksightTheme_Configuration_Sheet {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Sheet {
        DataQuicksightTheme_Configuration_Sheet {
            tile: core::default::Default::default(),
            tile_layout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_SheetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_SheetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightTheme_Configuration_SheetRef {
        DataQuicksightTheme_Configuration_SheetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_SheetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tile` after provisioning.\n"]
    pub fn tile(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_Sheet_TileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tile", self.base))
    }

    #[doc = "Get a reference to the value of field `tile_layout` after provisioning.\n"]
    pub fn tile_layout(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_Sheet_TileLayoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tile_layout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration_Typography_FontFamilies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightTheme_Configuration_Typography_FontFamilies {
    #[doc = "Set the field `font_family`.\n"]
    pub fn set_font_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.font_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Typography_FontFamilies {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Typography_FontFamilies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Typography_FontFamilies {}

impl BuildDataQuicksightTheme_Configuration_Typography_FontFamilies {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Typography_FontFamilies {
        DataQuicksightTheme_Configuration_Typography_FontFamilies { font_family: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_Typography_FontFamiliesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_Typography_FontFamiliesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_Typography_FontFamiliesRef {
        DataQuicksightTheme_Configuration_Typography_FontFamiliesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_Typography_FontFamiliesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `font_family` after provisioning.\n"]
    pub fn font_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.font_family", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration_Typography {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    font_families: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Typography_FontFamilies>>,
}

impl DataQuicksightTheme_Configuration_Typography {
    #[doc = "Set the field `font_families`.\n"]
    pub fn set_font_families(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Typography_FontFamilies>>,
    ) -> Self {
        self.font_families = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_Typography {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_Typography>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_Typography {}

impl BuildDataQuicksightTheme_Configuration_Typography {
    pub fn build(self) -> DataQuicksightTheme_Configuration_Typography {
        DataQuicksightTheme_Configuration_Typography { font_families: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_Configuration_TypographyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_TypographyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_TypographyRef {
        DataQuicksightTheme_Configuration_TypographyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_TypographyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `font_families` after provisioning.\n"]
    pub fn font_families(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_Typography_FontFamiliesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.font_families", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration_UiColorPalette {
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

impl DataQuicksightTheme_Configuration_UiColorPalette {
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

impl terrars::ToListMappable for DataQuicksightTheme_Configuration_UiColorPalette {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration_UiColorPalette>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration_UiColorPalette {}

impl BuildDataQuicksightTheme_Configuration_UiColorPalette {
    pub fn build(self) -> DataQuicksightTheme_Configuration_UiColorPalette {
        DataQuicksightTheme_Configuration_UiColorPalette {
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
pub struct DataQuicksightTheme_Configuration_UiColorPaletteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_Configuration_UiColorPaletteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightTheme_Configuration_UiColorPaletteRef {
        DataQuicksightTheme_Configuration_UiColorPaletteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_Configuration_UiColorPaletteRef {
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

#[derive(Serialize)]
pub struct DataQuicksightTheme_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_color_palette: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_DataColorPalette>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sheet: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Sheet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    typography: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_Typography>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_color_palette: std::option::Option<terrars::ListField<DataQuicksightTheme_Configuration_UiColorPalette>>,
}

impl DataQuicksightTheme_Configuration {
    #[doc = "Set the field `data_color_palette`.\n"]
    pub fn set_data_color_palette(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_DataColorPalette>>,
    ) -> Self {
        self.data_color_palette = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sheet`.\n"]
    pub fn set_sheet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Sheet>>,
    ) -> Self {
        self.sheet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `typography`.\n"]
    pub fn set_typography(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_Typography>>,
    ) -> Self {
        self.typography = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ui_color_palette`.\n"]
    pub fn set_ui_color_palette(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightTheme_Configuration_UiColorPalette>>,
    ) -> Self {
        self.ui_color_palette = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Configuration {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Configuration {}

impl BuildDataQuicksightTheme_Configuration {
    pub fn build(self) -> DataQuicksightTheme_Configuration {
        DataQuicksightTheme_Configuration {
            data_color_palette: core::default::Default::default(),
            sheet: core::default::Default::default(),
            typography: core::default::Default::default(),
            ui_color_palette: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightTheme_ConfigurationRef {
        DataQuicksightTheme_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_color_palette` after provisioning.\n"]
    pub fn data_color_palette(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_DataColorPaletteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_color_palette", self.base))
    }

    #[doc = "Get a reference to the value of field `sheet` after provisioning.\n"]
    pub fn sheet(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_SheetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sheet", self.base))
    }

    #[doc = "Get a reference to the value of field `typography` after provisioning.\n"]
    pub fn typography(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_TypographyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.typography", self.base))
    }

    #[doc = "Get a reference to the value of field `ui_color_palette` after provisioning.\n"]
    pub fn ui_color_palette(&self) -> terrars::ListRef<DataQuicksightTheme_Configuration_UiColorPaletteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_color_palette", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightTheme_Permissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightTheme_Permissions {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightTheme_Permissions {
    type O = terrars::BlockAssignable<DataQuicksightTheme_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightTheme_Permissions {}

impl BuildDataQuicksightTheme_Permissions {
    pub fn build(self) -> DataQuicksightTheme_Permissions {
        DataQuicksightTheme_Permissions {
            actions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightTheme_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightTheme_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightTheme_PermissionsRef {
        DataQuicksightTheme_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightTheme_PermissionsRef {
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
