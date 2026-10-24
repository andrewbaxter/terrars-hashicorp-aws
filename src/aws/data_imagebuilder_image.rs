use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderImageData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataImagebuilderImage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderImageData>,
}

#[derive(Clone)]
pub struct DataImagebuilderImage(std::rc::Rc<DataImagebuilderImage_>);

impl DataImagebuilderImage {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `build_version_arn` after provisioning.\n"]
    pub fn build_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_image_metadata_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<DataImagebuilderImageImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_resources` after provisioning.\n"]
    pub fn output_resources(&self) -> terrars::ListRef<DataImagebuilderImageOutputResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataImagebuilderImage {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImagebuilderImage { }

impl terrars::ToListMappable for DataImagebuilderImage {
    type O = terrars::ListRef<DataImagebuilderImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImagebuilderImage_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_imagebuilder_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderImage {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataImagebuilderImage {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImagebuilderImage {
        let out = DataImagebuilderImage(std::rc::Rc::new(DataImagebuilderImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataImagebuilderImageData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderImageRef {
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

    #[doc = "Get a reference to the value of field `build_version_arn` after provisioning.\n"]
    pub fn build_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_version_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_image_metadata_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<DataImagebuilderImageImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_version` after provisioning.\n"]
    pub fn os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_resources` after provisioning.\n"]
    pub fn output_resources(&self) -> terrars::ListRef<DataImagebuilderImageOutputResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataImagebuilderImageImageTestsConfigurationEl {
    #[doc = "Set the field `image_tests_enabled`.\n"]
    pub fn set_image_tests_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.image_tests_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderImageImageTestsConfigurationEl {
    type O = terrars::BlockAssignable<DataImagebuilderImageImageTestsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageImageTestsConfigurationEl {}

impl BuildDataImagebuilderImageImageTestsConfigurationEl {
    pub fn build(self) -> DataImagebuilderImageImageTestsConfigurationEl {
        DataImagebuilderImageImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageImageTestsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImageImageTestsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderImageImageTestsConfigurationElRef {
        DataImagebuilderImageImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageImageTestsConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_tests_enabled` after provisioning.\n"]
    pub fn image_tests_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tests_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageOutputResourcesElAmisEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderImageOutputResourcesElAmisEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderImageOutputResourcesElAmisEl {
    type O = terrars::BlockAssignable<DataImagebuilderImageOutputResourcesElAmisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageOutputResourcesElAmisEl {}

impl BuildDataImagebuilderImageOutputResourcesElAmisEl {
    pub fn build(self) -> DataImagebuilderImageOutputResourcesElAmisEl {
        DataImagebuilderImageOutputResourcesElAmisEl {
            account_id: core::default::Default::default(),
            description: core::default::Default::default(),
            image: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImageOutputResourcesElAmisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImageOutputResourcesElAmisElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderImageOutputResourcesElAmisElRef {
        DataImagebuilderImageOutputResourcesElAmisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageOutputResourcesElAmisElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImageOutputResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amis: std::option::Option<terrars::SetField<DataImagebuilderImageOutputResourcesElAmisEl>>,
}

impl DataImagebuilderImageOutputResourcesEl {
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataImagebuilderImageOutputResourcesElAmisEl>>,
    ) -> Self {
        self.amis = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderImageOutputResourcesEl {
    type O = terrars::BlockAssignable<DataImagebuilderImageOutputResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImageOutputResourcesEl {}

impl BuildDataImagebuilderImageOutputResourcesEl {
    pub fn build(self) -> DataImagebuilderImageOutputResourcesEl {
        DataImagebuilderImageOutputResourcesEl { amis: core::default::Default::default() }
    }
}

pub struct DataImagebuilderImageOutputResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImageOutputResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataImagebuilderImageOutputResourcesElRef {
        DataImagebuilderImageOutputResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImageOutputResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> terrars::SetRef<DataImagebuilderImageOutputResourcesElAmisElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.amis", self.base))
    }
}
