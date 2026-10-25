#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcrpublicImagesData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_ids: std::option::Option<std::vec::Vec<DataEcrpublicImages_ImageIds>>,
    dynamic: DataEcrpublicImagesDynamic,
}

struct DataEcrpublicImages_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcrpublicImagesData_>,
}

#[derive(Clone)]
pub struct DataEcrpublicImages(std::rc::Rc<DataEcrpublicImages_>);

impl DataEcrpublicImages {
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

    #[doc =
        "Set the field `registry_id`.\nAWS account ID associated with the public registry that contains the repository. If not specified, the default public registry is assumed."]
    pub fn set_registry_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().registry_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_ids`.\n"]
    pub fn set_image_ids(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcrpublicImages_ImageIds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_ids = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_ids = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> terrars::ListRef<DataEcrpublicImages_ImagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_id` after provisioning.\nAWS account ID associated with the public registry that contains the repository. If not specified, the default public registry is assumed."]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_name` after provisioning.\nName of the public repository."]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> terrars::ListRef<DataEcrpublicImages_ImageIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_ids", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcrpublicImages {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcrpublicImages { }

impl terrars::ToListMappable for DataEcrpublicImages {
    type O = terrars::ListRef<DataEcrpublicImagesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcrpublicImages_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecrpublic_images".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrpublicImages {
    pub tf_id: std::string::String,
    #[doc = "Name of the public repository."]
    pub repository_name: terrars::PrimField<std::string::String>,
}

impl BuildDataEcrpublicImages {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcrpublicImages {
        let out = DataEcrpublicImages(std::rc::Rc::new(DataEcrpublicImages_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcrpublicImagesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                registry_id: core::default::Default::default(),
                repository_name: self.repository_name,
                image_ids: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrpublicImagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrpublicImagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcrpublicImagesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> terrars::ListRef<DataEcrpublicImages_ImagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registry_id` after provisioning.\nAWS account ID associated with the public registry that contains the repository. If not specified, the default public registry is assumed."]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `repository_name` after provisioning.\nName of the public repository."]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_ids` after provisioning.\n"]
    pub fn image_ids(&self) -> terrars::ListRef<DataEcrpublicImages_ImageIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_ids", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEcrpublicImagesDynamic {
    image_ids: std::option::Option<terrars::DynamicBlock<DataEcrpublicImages_ImageIds>>,
}

#[derive(Serialize)]
pub struct DataEcrpublicImages_Images {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_media_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_manifest_media_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pushed_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_size_in_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcrpublicImages_Images {
    #[doc = "Set the field `artifact_media_type`.\n"]
    pub fn set_artifact_media_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.artifact_media_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_digest`.\n"]
    pub fn set_image_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_manifest_media_type`.\n"]
    pub fn set_image_manifest_media_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_manifest_media_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_pushed_at`.\n"]
    pub fn set_image_pushed_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_pushed_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_size_in_bytes`.\n"]
    pub fn set_image_size_in_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_size_in_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_tags`.\n"]
    pub fn set_image_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.image_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_id`.\n"]
    pub fn set_registry_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrpublicImages_Images {
    type O = terrars::BlockAssignable<DataEcrpublicImages_Images>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrpublicImages_Images {}

impl BuildDataEcrpublicImages_Images {
    pub fn build(self) -> DataEcrpublicImages_Images {
        DataEcrpublicImages_Images {
            artifact_media_type: core::default::Default::default(),
            image_digest: core::default::Default::default(),
            image_manifest_media_type: core::default::Default::default(),
            image_pushed_at: core::default::Default::default(),
            image_size_in_bytes: core::default::Default::default(),
            image_tags: core::default::Default::default(),
            registry_id: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrpublicImages_ImagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrpublicImages_ImagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcrpublicImages_ImagesRef {
        DataEcrpublicImages_ImagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrpublicImages_ImagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifact_media_type` after provisioning.\n"]
    pub fn artifact_media_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_media_type", self.base))
    }

    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\n"]
    pub fn image_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }

    #[doc = "Get a reference to the value of field `image_manifest_media_type` after provisioning.\n"]
    pub fn image_manifest_media_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_manifest_media_type", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pushed_at` after provisioning.\n"]
    pub fn image_pushed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_pushed_at", self.base))
    }

    #[doc = "Get a reference to the value of field `image_size_in_bytes` after provisioning.\n"]
    pub fn image_size_in_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_size_in_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `image_tags` after provisioning.\n"]
    pub fn image_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrpublicImages_ImageIds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcrpublicImages_ImageIds {
    #[doc = "Set the field `image_digest`.\nImage digest."]
    pub fn set_image_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_tag`.\nImage tag."]
    pub fn set_image_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrpublicImages_ImageIds {
    type O = terrars::BlockAssignable<DataEcrpublicImages_ImageIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrpublicImages_ImageIds {}

impl BuildDataEcrpublicImages_ImageIds {
    pub fn build(self) -> DataEcrpublicImages_ImageIds {
        DataEcrpublicImages_ImageIds {
            image_digest: core::default::Default::default(),
            image_tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrpublicImages_ImageIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrpublicImages_ImageIdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcrpublicImages_ImageIdsRef {
        DataEcrpublicImages_ImageIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrpublicImages_ImageIdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\nImage digest."]
    pub fn image_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }

    #[doc = "Get a reference to the value of field `image_tag` after provisioning.\nImage tag."]
    pub fn image_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag", self.base))
    }
}
