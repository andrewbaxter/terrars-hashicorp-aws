#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcrImagesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_id: std::option::Option<terrars::PrimField<std::string::String>>,
    repository_name: terrars::PrimField<std::string::String>,
}

struct DataEcrImages_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcrImagesData_>,
}

#[derive(Clone)]
pub struct DataEcrImages(std::rc::Rc<DataEcrImages_>);

impl DataEcrImages {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_id`.\nID of the registry (AWS account ID)"]
    pub fn set_registry_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().registry_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> terrars::ListRef<DataEcrImages_ImageIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_id` after provisioning.\nID of the registry (AWS account ID)"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\nName of the repository"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcrImages {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcrImages { }

impl terrars::ToListMappable for DataEcrImages {
    type O = terrars::ListRef<DataEcrImagesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcrImages_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecr_images".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrImages {
    pub tf_id: std::string::String,
    #[doc = "Name of the repository"]
    pub repository_name: terrars::PrimField<std::string::String>,
}

impl BuildDataEcrImages {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcrImages {
        let out = DataEcrImages(std::rc::Rc::new(DataEcrImages_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcrImagesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                registry_id: core::default::Default::default(),
                repository_name: self.repository_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrImagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrImagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcrImagesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> terrars::ListRef<DataEcrImages_ImageIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_id` after provisioning.\nID of the registry (AWS account ID)"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\nName of the repository"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcrImages_ImageIds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcrImages_ImageIds {
    #[doc = "Set the field `image_digest`.\n"]
    pub fn set_image_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_tag`.\n"]
    pub fn set_image_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrImages_ImageIds {
    type O = terrars::BlockAssignable<DataEcrImages_ImageIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrImages_ImageIds {}

impl BuildDataEcrImages_ImageIds {
    pub fn build(self) -> DataEcrImages_ImageIds {
        DataEcrImages_ImageIds {
            image_digest: core::default::Default::default(),
            image_tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrImages_ImageIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrImages_ImageIdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcrImages_ImageIdsRef {
        DataEcrImages_ImageIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrImages_ImageIdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\n"]
    pub fn image_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }

    #[doc = "Get a reference to the value of field `image_tag` after provisioning.\n"]
    pub fn image_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag", self.base))
    }
}
