use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderContainerRecipeData {
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

struct DataImagebuilderContainerRecipe_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderContainerRecipeData>,
}

#[derive(Clone)]
pub struct DataImagebuilderContainerRecipe(std::rc::Rc<DataImagebuilderContainerRecipe_>);

impl DataImagebuilderContainerRecipe {
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

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataImagebuilderContainerRecipeComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> terrars::ListRef<DataImagebuilderContainerRecipeInstanceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> terrars::ListRef<DataImagebuilderContainerRecipeTargetRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }
}

impl terrars::Referable for DataImagebuilderContainerRecipe {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImagebuilderContainerRecipe { }

impl terrars::ToListMappable for DataImagebuilderContainerRecipe {
    type O = terrars::ListRef<DataImagebuilderContainerRecipeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImagebuilderContainerRecipe_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_imagebuilder_container_recipe".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderContainerRecipe {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataImagebuilderContainerRecipe {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImagebuilderContainerRecipe {
        let out = DataImagebuilderContainerRecipe(std::rc::Rc::new(DataImagebuilderContainerRecipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataImagebuilderContainerRecipeData {
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

pub struct DataImagebuilderContainerRecipeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderContainerRecipeRef {
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

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataImagebuilderContainerRecipeComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> terrars::ListRef<DataImagebuilderContainerRecipeInstanceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(&self) -> terrars::ListRef<DataImagebuilderContainerRecipeTargetRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeComponentElParameterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderContainerRecipeComponentElParameterEl {
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

impl terrars::ToListMappable for DataImagebuilderContainerRecipeComponentElParameterEl {
    type O = terrars::BlockAssignable<DataImagebuilderContainerRecipeComponentElParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeComponentElParameterEl {}

impl BuildDataImagebuilderContainerRecipeComponentElParameterEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeComponentElParameterEl {
        DataImagebuilderContainerRecipeComponentElParameterEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeComponentElParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeComponentElParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderContainerRecipeComponentElParameterElRef {
        DataImagebuilderContainerRecipeComponentElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeComponentElParameterElRef {
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

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::SetField<DataImagebuilderContainerRecipeComponentElParameterEl>>,
}

impl DataImagebuilderContainerRecipeComponentEl {
    #[doc = "Set the field `component_arn`.\n"]
    pub fn set_component_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.component_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataImagebuilderContainerRecipeComponentElParameterEl>>,
    ) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderContainerRecipeComponentEl {
    type O = terrars::BlockAssignable<DataImagebuilderContainerRecipeComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeComponentEl {}

impl BuildDataImagebuilderContainerRecipeComponentEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeComponentEl {
        DataImagebuilderContainerRecipeComponentEl {
            component_arn: core::default::Default::default(),
            parameter: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeComponentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataImagebuilderContainerRecipeComponentElRef {
        DataImagebuilderContainerRecipeComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeComponentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> terrars::SetRef<DataImagebuilderContainerRecipeComponentElParameterElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
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

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
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

impl terrars::ToListMappable for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    type O =
        terrars::BlockAssignable<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {}

impl BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl {
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

pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
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

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs: std
    ::option
    ::Option<terrars::ListField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
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
                        ::ListField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsEl>,
                    >,
    ) -> Self {
        self.ebs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.no_device = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    type O = terrars::BlockAssignable<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {}

impl BuildDataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl {
            device_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
        DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(
        &self,
    ) -> terrars::ListRef<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElEbsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeInstanceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_device_mapping: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderContainerRecipeInstanceConfigurationEl {
    #[doc = "Set the field `block_device_mapping`.\n"]
    pub fn set_block_device_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingEl>,
                    >,
    ) -> Self {
        self.block_device_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderContainerRecipeInstanceConfigurationEl {
    type O = terrars::BlockAssignable<DataImagebuilderContainerRecipeInstanceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeInstanceConfigurationEl {}

impl BuildDataImagebuilderContainerRecipeInstanceConfigurationEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeInstanceConfigurationEl {
        DataImagebuilderContainerRecipeInstanceConfigurationEl {
            block_device_mapping: core::default::Default::default(),
            image: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeInstanceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderContainerRecipeInstanceConfigurationElRef {
        DataImagebuilderContainerRecipeInstanceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeInstanceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_device_mapping` after provisioning.\n"]
    pub fn block_device_mapping(
        &self,
    ) -> terrars::SetRef<DataImagebuilderContainerRecipeInstanceConfigurationElBlockDeviceMappingElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.block_device_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipeTargetRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderContainerRecipeTargetRepositoryEl {
    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderContainerRecipeTargetRepositoryEl {
    type O = terrars::BlockAssignable<DataImagebuilderContainerRecipeTargetRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipeTargetRepositoryEl {}

impl BuildDataImagebuilderContainerRecipeTargetRepositoryEl {
    pub fn build(self) -> DataImagebuilderContainerRecipeTargetRepositoryEl {
        DataImagebuilderContainerRecipeTargetRepositoryEl {
            repository_name: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderContainerRecipeTargetRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderContainerRecipeTargetRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderContainerRecipeTargetRepositoryElRef {
        DataImagebuilderContainerRecipeTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipeTargetRepositoryElRef {
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
