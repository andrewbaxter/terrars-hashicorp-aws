use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderImageData {
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
    container_recipe_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_image_metadata_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_recipe_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    infrastructure_configuration_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_configuration: std::option::Option<std::vec::Vec<ImagebuilderImageImageTestsConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ImagebuilderImageTimeoutsEl>,
    dynamic: ImagebuilderImageDynamic,
}

struct ImagebuilderImage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderImageData>,
}

#[derive(Clone)]
pub struct ImagebuilderImage(std::rc::Rc<ImagebuilderImage_>);

impl ImagebuilderImage {
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

    #[doc = "Set the field `container_recipe_arn`.\n"]
    pub fn set_container_recipe_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().container_recipe_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `distribution_configuration_arn`.\n"]
    pub fn set_distribution_configuration_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().distribution_configuration_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enhanced_image_metadata_enabled`.\n"]
    pub fn set_enhanced_image_metadata_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_image_metadata_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_recipe_arn`.\n"]
    pub fn set_image_recipe_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_recipe_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `image_tests_configuration`.\n"]
    pub fn set_image_tests_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImageImageTestsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_tests_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_tests_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ImagebuilderImageTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    pub fn output_resources(&self) -> terrars::ListRef<ImagebuilderImageOutputResourcesElRef> {
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<ImagebuilderImageImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ImagebuilderImageTimeoutsElRef {
        ImagebuilderImageTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ImagebuilderImage {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImagebuilderImage { }

impl terrars::ToListMappable for ImagebuilderImage {
    type O = terrars::ListRef<ImagebuilderImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImagebuilderImage_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_imagebuilder_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderImage {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub infrastructure_configuration_arn: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImage {
    pub fn build(self, stack: &mut terrars::Stack) -> ImagebuilderImage {
        let out = ImagebuilderImage(std::rc::Rc::new(ImagebuilderImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImagebuilderImageData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_recipe_arn: core::default::Default::default(),
                distribution_configuration_arn: core::default::Default::default(),
                enhanced_image_metadata_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                image_recipe_arn: core::default::Default::default(),
                infrastructure_configuration_arn: self.infrastructure_configuration_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                image_tests_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderImageRef {
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
    pub fn output_resources(&self) -> terrars::ListRef<ImagebuilderImageOutputResourcesElRef> {
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<ImagebuilderImageImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ImagebuilderImageTimeoutsElRef {
        ImagebuilderImageTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageOutputResourcesElAmisEl {
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

impl ImagebuilderImageOutputResourcesElAmisEl {
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

impl terrars::ToListMappable for ImagebuilderImageOutputResourcesElAmisEl {
    type O = terrars::BlockAssignable<ImagebuilderImageOutputResourcesElAmisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageOutputResourcesElAmisEl {}

impl BuildImagebuilderImageOutputResourcesElAmisEl {
    pub fn build(self) -> ImagebuilderImageOutputResourcesElAmisEl {
        ImagebuilderImageOutputResourcesElAmisEl {
            account_id: core::default::Default::default(),
            description: core::default::Default::default(),
            image: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImageOutputResourcesElAmisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageOutputResourcesElAmisElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImageOutputResourcesElAmisElRef {
        ImagebuilderImageOutputResourcesElAmisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageOutputResourcesElAmisElRef {
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
pub struct ImagebuilderImageOutputResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amis: std::option::Option<terrars::SetField<ImagebuilderImageOutputResourcesElAmisEl>>,
}

impl ImagebuilderImageOutputResourcesEl {
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(
        mut self,
        v: impl std::convert::Into<terrars::SetField<ImagebuilderImageOutputResourcesElAmisEl>>,
    ) -> Self {
        self.amis = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImageOutputResourcesEl {
    type O = terrars::BlockAssignable<ImagebuilderImageOutputResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageOutputResourcesEl {}

impl BuildImagebuilderImageOutputResourcesEl {
    pub fn build(self) -> ImagebuilderImageOutputResourcesEl {
        ImagebuilderImageOutputResourcesEl { amis: core::default::Default::default() }
    }
}

pub struct ImagebuilderImageOutputResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageOutputResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImageOutputResourcesElRef {
        ImagebuilderImageOutputResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageOutputResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> terrars::SetRef<ImagebuilderImageOutputResourcesElAmisElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.amis", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ImagebuilderImageImageTestsConfigurationEl {
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

impl terrars::ToListMappable for ImagebuilderImageImageTestsConfigurationEl {
    type O = terrars::BlockAssignable<ImagebuilderImageImageTestsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageImageTestsConfigurationEl {}

impl BuildImagebuilderImageImageTestsConfigurationEl {
    pub fn build(self) -> ImagebuilderImageImageTestsConfigurationEl {
        ImagebuilderImageImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImageImageTestsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageImageTestsConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImageImageTestsConfigurationElRef {
        ImagebuilderImageImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageImageTestsConfigurationElRef {
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
pub struct ImagebuilderImageTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderImageTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImageTimeoutsEl {
    type O = terrars::BlockAssignable<ImagebuilderImageTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageTimeoutsEl {}

impl BuildImagebuilderImageTimeoutsEl {
    pub fn build(self) -> ImagebuilderImageTimeoutsEl {
        ImagebuilderImageTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct ImagebuilderImageTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImageTimeoutsElRef {
        ImagebuilderImageTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImageDynamic {
    image_tests_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderImageImageTestsConfigurationEl>>,
}
