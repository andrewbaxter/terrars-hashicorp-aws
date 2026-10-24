use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderImageRecipeData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_device_mapping: std::option::Option<std::vec::Vec<ImagebuilderImageRecipeBlockDeviceMappingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<std::vec::Vec<ImagebuilderImageRecipeComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    systems_manager_agent: std::option::Option<std::vec::Vec<ImagebuilderImageRecipeSystemsManagerAgentEl>>,
    dynamic: ImagebuilderImageRecipeDynamic,
}

struct ImagebuilderImageRecipe_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderImageRecipeData>,
}

#[derive(Clone)]
pub struct ImagebuilderImageRecipe(std::rc::Rc<ImagebuilderImageRecipe_>);

impl ImagebuilderImageRecipe {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_base64`.\n"]
    pub fn set_user_data_base64(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_data_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().working_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_device_mapping`.\n"]
    pub fn set_block_device_mapping(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_device_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_device_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImageRecipeComponentEl>>,
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

    #[doc = "Set the field `systems_manager_agent`.\n"]
    pub fn set_systems_manager_agent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImageRecipeSystemsManagerAgentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().systems_manager_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.systems_manager_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
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
    pub fn component(&self) -> terrars::ListRef<ImagebuilderImageRecipeComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `systems_manager_agent` after provisioning.\n"]
    pub fn systems_manager_agent(&self) -> terrars::ListRef<ImagebuilderImageRecipeSystemsManagerAgentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.systems_manager_agent", self.extract_ref()))
    }
}

impl terrars::Referable for ImagebuilderImageRecipe {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImagebuilderImageRecipe { }

impl terrars::ToListMappable for ImagebuilderImageRecipe {
    type O = terrars::ListRef<ImagebuilderImageRecipeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImagebuilderImageRecipe_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_imagebuilder_image_recipe".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderImageRecipe {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parent_image: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImageRecipe {
    pub fn build(self, stack: &mut terrars::Stack) -> ImagebuilderImageRecipe {
        let out = ImagebuilderImageRecipe(std::rc::Rc::new(ImagebuilderImageRecipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImagebuilderImageRecipeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent_image: self.parent_image,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_data_base64: core::default::Default::default(),
                version: self.version,
                working_directory: core::default::Default::default(),
                block_device_mapping: core::default::Default::default(),
                component: core::default::Default::default(),
                systems_manager_agent: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderImageRecipeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRecipeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderImageRecipeRef {
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

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
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
    pub fn component(&self) -> terrars::ListRef<ImagebuilderImageRecipeComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `systems_manager_agent` after provisioning.\n"]
    pub fn systems_manager_agent(&self) -> terrars::ListRef<ImagebuilderImageRecipeSystemsManagerAgentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.systems_manager_agent", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
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

impl ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
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

impl terrars::ToListMappable for ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
    type O = terrars::BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeBlockDeviceMappingElEbsEl {}

impl BuildImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
    pub fn build(self) -> ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
        ImagebuilderImageRecipeBlockDeviceMappingElEbsEl {
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

pub struct ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
        ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef {
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
struct ImagebuilderImageRecipeBlockDeviceMappingElDynamic {
    ebs: std::option::Option<terrars::DynamicBlock<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeBlockDeviceMappingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs: std::option::Option<std::vec::Vec<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>>,
    dynamic: ImagebuilderImageRecipeBlockDeviceMappingElDynamic,
}

impl ImagebuilderImageRecipeBlockDeviceMappingEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingElEbsEl>>,
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

impl terrars::ToListMappable for ImagebuilderImageRecipeBlockDeviceMappingEl {
    type O = terrars::BlockAssignable<ImagebuilderImageRecipeBlockDeviceMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeBlockDeviceMappingEl {}

impl BuildImagebuilderImageRecipeBlockDeviceMappingEl {
    pub fn build(self) -> ImagebuilderImageRecipeBlockDeviceMappingEl {
        ImagebuilderImageRecipeBlockDeviceMappingEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImageRecipeBlockDeviceMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRecipeBlockDeviceMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImageRecipeBlockDeviceMappingElRef {
        ImagebuilderImageRecipeBlockDeviceMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeBlockDeviceMappingElRef {
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
    pub fn ebs(&self) -> terrars::ListRef<ImagebuilderImageRecipeBlockDeviceMappingElEbsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeComponentElParameterEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ImagebuilderImageRecipeComponentElParameterEl { }

impl terrars::ToListMappable for ImagebuilderImageRecipeComponentElParameterEl {
    type O = terrars::BlockAssignable<ImagebuilderImageRecipeComponentElParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeComponentElParameterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImageRecipeComponentElParameterEl {
    pub fn build(self) -> ImagebuilderImageRecipeComponentElParameterEl {
        ImagebuilderImageRecipeComponentElParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ImagebuilderImageRecipeComponentElParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRecipeComponentElParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImageRecipeComponentElParameterElRef {
        ImagebuilderImageRecipeComponentElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeComponentElParameterElRef {
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
struct ImagebuilderImageRecipeComponentElDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<ImagebuilderImageRecipeComponentElParameterEl>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeComponentEl {
    component_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<ImagebuilderImageRecipeComponentElParameterEl>>,
    dynamic: ImagebuilderImageRecipeComponentElDynamic,
}

impl ImagebuilderImageRecipeComponentEl {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImageRecipeComponentElParameterEl>>,
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

impl terrars::ToListMappable for ImagebuilderImageRecipeComponentEl {
    type O = terrars::BlockAssignable<ImagebuilderImageRecipeComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeComponentEl {
    #[doc = ""]
    pub component_arn: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImageRecipeComponentEl {
    pub fn build(self) -> ImagebuilderImageRecipeComponentEl {
        ImagebuilderImageRecipeComponentEl {
            component_arn: self.component_arn,
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImageRecipeComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRecipeComponentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImageRecipeComponentElRef {
        ImagebuilderImageRecipeComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeComponentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImageRecipeSystemsManagerAgentEl {
    uninstall_after_build: terrars::PrimField<bool>,
}

impl ImagebuilderImageRecipeSystemsManagerAgentEl { }

impl terrars::ToListMappable for ImagebuilderImageRecipeSystemsManagerAgentEl {
    type O = terrars::BlockAssignable<ImagebuilderImageRecipeSystemsManagerAgentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImageRecipeSystemsManagerAgentEl {
    #[doc = ""]
    pub uninstall_after_build: terrars::PrimField<bool>,
}

impl BuildImagebuilderImageRecipeSystemsManagerAgentEl {
    pub fn build(self) -> ImagebuilderImageRecipeSystemsManagerAgentEl {
        ImagebuilderImageRecipeSystemsManagerAgentEl { uninstall_after_build: self.uninstall_after_build }
    }
}

pub struct ImagebuilderImageRecipeSystemsManagerAgentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImageRecipeSystemsManagerAgentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImageRecipeSystemsManagerAgentElRef {
        ImagebuilderImageRecipeSystemsManagerAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImageRecipeSystemsManagerAgentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uninstall_after_build` after provisioning.\n"]
    pub fn uninstall_after_build(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uninstall_after_build", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImageRecipeDynamic {
    block_device_mapping: std::option::Option<terrars::DynamicBlock<ImagebuilderImageRecipeBlockDeviceMappingEl>>,
    component: std::option::Option<terrars::DynamicBlock<ImagebuilderImageRecipeComponentEl>>,
    systems_manager_agent: std::option::Option<terrars::DynamicBlock<ImagebuilderImageRecipeSystemsManagerAgentEl>>,
}
