use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderContainerRecipeData {
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
    container_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_template_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_template_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<std::vec::Vec<ImagebuilderContainerRecipeComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_configuration: std::option::Option<std::vec::Vec<ImagebuilderContainerRecipeInstanceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_repository: std::option::Option<std::vec::Vec<ImagebuilderContainerRecipeTargetRepositoryEl>>,
    dynamic: ImagebuilderContainerRecipeDynamic,
}

struct ImagebuilderContainerRecipe_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderContainerRecipeData>,
}

#[derive(Clone)]
pub struct ImagebuilderContainerRecipe(std::rc::Rc<ImagebuilderContainerRecipe_>);

impl ImagebuilderContainerRecipe {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_template_data`.\n"]
    pub fn set_dockerfile_template_data(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().dockerfile_template_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_template_uri`.\n"]
    pub fn set_dockerfile_template_uri(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().dockerfile_template_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().working_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderContainerRecipeComponentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().component = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.component = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_configuration`.\n"]
    pub fn set_instance_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_repository`.\n"]
    pub fn set_target_repository(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderContainerRecipeTargetRepositoryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_data` after provisioning.\n"]
    pub fn dockerfile_template_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_uri` after provisioning.\n"]
    pub fn dockerfile_template_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_template_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_image` after provisioning.\n"]
    pub fn parent_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_image", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<ImagebuilderContainerRecipeComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> terrars::ListRef<ImagebuilderContainerRecipeInstanceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> terrars::ListRef<ImagebuilderContainerRecipeTargetRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_repository", self.extract_ref()))
    }
}

impl terrars::Referable for ImagebuilderContainerRecipe {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImagebuilderContainerRecipe { }

impl terrars::ToListMappable for ImagebuilderContainerRecipe {
    type O = terrars::ListRef<ImagebuilderContainerRecipeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImagebuilderContainerRecipe_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_imagebuilder_container_recipe".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderContainerRecipe {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub container_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parent_image: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderContainerRecipe {
    pub fn build(self, stack: &mut terrars::Stack) -> ImagebuilderContainerRecipe {
        let out = ImagebuilderContainerRecipe(std::rc::Rc::new(ImagebuilderContainerRecipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImagebuilderContainerRecipeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_type: self.container_type,
                description: core::default::Default::default(),
                dockerfile_template_data: core::default::Default::default(),
                dockerfile_template_uri: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                name: self.name,
                parent_image: self.parent_image,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: self.version,
                working_directory: core::default::Default::default(),
                component: core::default::Default::default(),
                instance_configuration: core::default::Default::default(),
                target_repository: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderContainerRecipeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderContainerRecipeRef {
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

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_data` after provisioning.\n"]
    pub fn dockerfile_template_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dockerfile_template_uri` after provisioning.\n"]
    pub fn dockerfile_template_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_template_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_image` after provisioning.\n"]
    pub fn parent_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_image", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<ImagebuilderContainerRecipeComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> terrars::ListRef<ImagebuilderContainerRecipeInstanceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> terrars::ListRef<ImagebuilderContainerRecipeTargetRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeComponentElParameterEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ImagebuilderContainerRecipeComponentElParameterEl { }

impl terrars::ToListMappable for ImagebuilderContainerRecipeComponentElParameterEl {
    type O = terrars::BlockAssignable<ImagebuilderContainerRecipeComponentElParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeComponentElParameterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderContainerRecipeComponentElParameterEl {
    pub fn build(self) -> ImagebuilderContainerRecipeComponentElParameterEl {
        ImagebuilderContainerRecipeComponentElParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ImagebuilderContainerRecipeComponentElParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeComponentElParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderContainerRecipeComponentElParameterElRef {
        ImagebuilderContainerRecipeComponentElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeComponentElParameterElRef {
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
struct ImagebuilderContainerRecipeComponentElDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<ImagebuilderContainerRecipeComponentElParameterEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeComponentEl {
    component_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<ImagebuilderContainerRecipeComponentElParameterEl>>,
    dynamic: ImagebuilderContainerRecipeComponentElDynamic,
}

impl ImagebuilderContainerRecipeComponentEl {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderContainerRecipeComponentElParameterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderContainerRecipeComponentEl {
    type O = terrars::BlockAssignable<ImagebuilderContainerRecipeComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeComponentEl {
    #[doc = ""]
    pub component_arn: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderContainerRecipeComponentEl {
    pub fn build(self) -> ImagebuilderContainerRecipeComponentEl {
        ImagebuilderContainerRecipeComponentEl {
            component_arn: self.component_arn,
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeComponentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderContainerRecipeComponentElRef {
        ImagebuilderContainerRecipeComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeComponentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    type O = terrars::BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {}

impl BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    pub fn build(self) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElDynamic {
    ebs: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs: std::option::Option<std::vec::Vec<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>>,
    dynamic: ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElDynamic,
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_device = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    type O = terrars::BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {}

impl BuildImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    pub fn build(self) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
        ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(
        &self,
    ) -> terrars::ListRef<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeInstanceConfigurationElDynamic {
    block_device_mapping: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeInstanceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_device_mapping: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>>,
    dynamic: ImagebuilderContainerRecipeInstanceConfigurationElDynamic,
}

impl ImagebuilderContainerRecipeInstanceConfigurationEl {
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_device_mapping`.\n"]
    pub fn set_block_device_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block_device_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block_device_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderContainerRecipeInstanceConfigurationEl {
    type O = terrars::BlockAssignable<ImagebuilderContainerRecipeInstanceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeInstanceConfigurationEl {}

impl BuildImagebuilderContainerRecipeInstanceConfigurationEl {
    pub fn build(self) -> ImagebuilderContainerRecipeInstanceConfigurationEl {
        ImagebuilderContainerRecipeInstanceConfigurationEl {
            image: core::default::Default::default(),
            block_device_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderContainerRecipeInstanceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderContainerRecipeInstanceConfigurationElRef {
        ImagebuilderContainerRecipeInstanceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderContainerRecipeTargetRepositoryEl {
    repository_name: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl ImagebuilderContainerRecipeTargetRepositoryEl { }

impl terrars::ToListMappable for ImagebuilderContainerRecipeTargetRepositoryEl {
    type O = terrars::BlockAssignable<ImagebuilderContainerRecipeTargetRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderContainerRecipeTargetRepositoryEl {
    #[doc = ""]
    pub repository_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderContainerRecipeTargetRepositoryEl {
    pub fn build(self) -> ImagebuilderContainerRecipeTargetRepositoryEl {
        ImagebuilderContainerRecipeTargetRepositoryEl {
            repository_name: self.repository_name,
            service: self.service,
        }
    }
}

pub struct ImagebuilderContainerRecipeTargetRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderContainerRecipeTargetRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderContainerRecipeTargetRepositoryElRef {
        ImagebuilderContainerRecipeTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderContainerRecipeTargetRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderContainerRecipeDynamic {
    component: std::option::Option<terrars::DynamicBlock<ImagebuilderContainerRecipeComponentEl>>,
    instance_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderContainerRecipeInstanceConfigurationEl>>,
    target_repository: std::option::Option<terrars::DynamicBlock<ImagebuilderContainerRecipeTargetRepositoryEl>>,
}
