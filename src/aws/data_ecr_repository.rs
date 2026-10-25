#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcrRepositoryData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEcrRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcrRepositoryData_>,
}

#[derive(Clone)]
pub struct DataEcrRepository(std::rc::Rc<DataEcrRepository_>);

impl DataEcrRepository {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_id`.\n"]
    pub fn set_registry_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().registry_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<DataEcrRepository_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> terrars::ListRef<DataEcrRepository_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> terrars::ListRef<DataEcrRepository_ImageTagMutabilityExclusionFilterRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `most_recent_image_tags` after provisioning.\n"]
    pub fn most_recent_image_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.most_recent_image_tags", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcrRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcrRepository { }

impl terrars::ToListMappable for DataEcrRepository {
    type O = terrars::ListRef<DataEcrRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcrRepository_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecr_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrRepository {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataEcrRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcrRepository {
        let out = DataEcrRepository(std::rc::Rc::new(DataEcrRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcrRepositoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                registry_id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcrRepositoryRef {
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<DataEcrRepository_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> terrars::ListRef<DataEcrRepository_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> terrars::ListRef<DataEcrRepository_ImageTagMutabilityExclusionFilterRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `most_recent_image_tags` after provisioning.\n"]
    pub fn most_recent_image_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.most_recent_image_tags", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcrRepository_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcrRepository_EncryptionConfiguration {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrRepository_EncryptionConfiguration {
    type O = terrars::BlockAssignable<DataEcrRepository_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepository_EncryptionConfiguration {}

impl BuildDataEcrRepository_EncryptionConfiguration {
    pub fn build(self) -> DataEcrRepository_EncryptionConfiguration {
        DataEcrRepository_EncryptionConfiguration {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrRepository_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrRepository_EncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcrRepository_EncryptionConfigurationRef {
        DataEcrRepository_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepository_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrRepository_ImageScanningConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_on_push: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEcrRepository_ImageScanningConfiguration {
    #[doc = "Set the field `scan_on_push`.\n"]
    pub fn set_scan_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.scan_on_push = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrRepository_ImageScanningConfiguration {
    type O = terrars::BlockAssignable<DataEcrRepository_ImageScanningConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepository_ImageScanningConfiguration {}

impl BuildDataEcrRepository_ImageScanningConfiguration {
    pub fn build(self) -> DataEcrRepository_ImageScanningConfiguration {
        DataEcrRepository_ImageScanningConfiguration { scan_on_push: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEcrRepository_ImageScanningConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrRepository_ImageScanningConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcrRepository_ImageScanningConfigurationRef {
        DataEcrRepository_ImageScanningConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepository_ImageScanningConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scan_on_push` after provisioning.\n"]
    pub fn scan_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrRepository_ImageTagMutabilityExclusionFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcrRepository_ImageTagMutabilityExclusionFilter {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_type`.\n"]
    pub fn set_filter_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcrRepository_ImageTagMutabilityExclusionFilter {
    type O = terrars::BlockAssignable<DataEcrRepository_ImageTagMutabilityExclusionFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrRepository_ImageTagMutabilityExclusionFilter {}

impl BuildDataEcrRepository_ImageTagMutabilityExclusionFilter {
    pub fn build(self) -> DataEcrRepository_ImageTagMutabilityExclusionFilter {
        DataEcrRepository_ImageTagMutabilityExclusionFilter {
            filter: core::default::Default::default(),
            filter_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcrRepository_ImageTagMutabilityExclusionFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcrRepository_ImageTagMutabilityExclusionFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcrRepository_ImageTagMutabilityExclusionFilterRef {
        DataEcrRepository_ImageTagMutabilityExclusionFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrRepository_ImageTagMutabilityExclusionFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}
