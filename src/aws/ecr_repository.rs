#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrRepositoryData_ {
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
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tag_mutability: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<EcrRepository_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_scanning_configuration: std::option::Option<std::vec::Vec<EcrRepository_ImageScanningConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tag_mutability_exclusion_filter: std
    ::option
    ::Option<std::vec::Vec<EcrRepository_ImageTagMutabilityExclusionFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EcrRepository_Timeouts>,
    dynamic: EcrRepositoryDynamic,
}

struct EcrRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcrRepositoryData_>,
}

#[derive(Clone)]
pub struct EcrRepository(std::rc::Rc<EcrRepository_>);

impl EcrRepository {
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

    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_tag_mutability`.\n"]
    pub fn set_image_tag_mutability(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_tag_mutability = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrRepository_EncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_scanning_configuration`.\n"]
    pub fn set_image_scanning_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrRepository_ImageScanningConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_scanning_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_scanning_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_tag_mutability_exclusion_filter`.\n"]
    pub fn set_image_tag_mutability_exclusion_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrRepository_ImageTagMutabilityExclusionFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_tag_mutability_exclusion_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_tag_mutability_exclusion_filter =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EcrRepository_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<EcrRepository_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> terrars::ListRef<EcrRepository_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> terrars::ListRef<EcrRepository_ImageTagMutabilityExclusionFilterRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrRepository_TimeoutsRef {
        EcrRepository_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EcrRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcrRepository { }

impl terrars::ToListMappable for EcrRepository {
    type O = terrars::ListRef<EcrRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcrRepository_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecr_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrRepository {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcrRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> EcrRepository {
        let out = EcrRepository(std::rc::Rc::new(EcrRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcrRepositoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                image_tag_mutability: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                image_scanning_configuration: core::default::Default::default(),
                image_tag_mutability_exclusion_filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcrRepositoryRef {
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

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<EcrRepository_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(&self) -> terrars::ListRef<EcrRepository_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> terrars::ListRef<EcrRepository_ImageTagMutabilityExclusionFilterRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrRepository_TimeoutsRef {
        EcrRepository_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcrRepositoryDynamic {
    encryption_configuration: std::option::Option<terrars::DynamicBlock<EcrRepository_EncryptionConfiguration>>,
    image_scanning_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcrRepository_ImageScanningConfiguration>>,
    image_tag_mutability_exclusion_filter: std
    ::option
    ::Option<terrars::DynamicBlock<EcrRepository_ImageTagMutabilityExclusionFilter>>,
}

#[derive(Serialize)]
pub struct EcrRepository_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcrRepository_EncryptionConfiguration {
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

impl terrars::ToListMappable for EcrRepository_EncryptionConfiguration {
    type O = terrars::BlockAssignable<EcrRepository_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRepository_EncryptionConfiguration {}

impl BuildEcrRepository_EncryptionConfiguration {
    pub fn build(self) -> EcrRepository_EncryptionConfiguration {
        EcrRepository_EncryptionConfiguration {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcrRepository_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepository_EncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcrRepository_EncryptionConfigurationRef {
        EcrRepository_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRepository_EncryptionConfigurationRef {
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
pub struct EcrRepository_ImageScanningConfiguration {
    scan_on_push: terrars::PrimField<bool>,
}

impl EcrRepository_ImageScanningConfiguration { }

impl terrars::ToListMappable for EcrRepository_ImageScanningConfiguration {
    type O = terrars::BlockAssignable<EcrRepository_ImageScanningConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRepository_ImageScanningConfiguration {
    #[doc = ""]
    pub scan_on_push: terrars::PrimField<bool>,
}

impl BuildEcrRepository_ImageScanningConfiguration {
    pub fn build(self) -> EcrRepository_ImageScanningConfiguration {
        EcrRepository_ImageScanningConfiguration { scan_on_push: self.scan_on_push }
    }
}

#[allow(dead_code)]
pub struct EcrRepository_ImageScanningConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepository_ImageScanningConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcrRepository_ImageScanningConfigurationRef {
        EcrRepository_ImageScanningConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRepository_ImageScanningConfigurationRef {
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
pub struct EcrRepository_ImageTagMutabilityExclusionFilter {
    filter: terrars::PrimField<std::string::String>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl EcrRepository_ImageTagMutabilityExclusionFilter { }

impl terrars::ToListMappable for EcrRepository_ImageTagMutabilityExclusionFilter {
    type O = terrars::BlockAssignable<EcrRepository_ImageTagMutabilityExclusionFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRepository_ImageTagMutabilityExclusionFilter {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildEcrRepository_ImageTagMutabilityExclusionFilter {
    pub fn build(self) -> EcrRepository_ImageTagMutabilityExclusionFilter {
        EcrRepository_ImageTagMutabilityExclusionFilter {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

#[allow(dead_code)]
pub struct EcrRepository_ImageTagMutabilityExclusionFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepository_ImageTagMutabilityExclusionFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrRepository_ImageTagMutabilityExclusionFilterRef {
        EcrRepository_ImageTagMutabilityExclusionFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRepository_ImageTagMutabilityExclusionFilterRef {
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

#[derive(Serialize)]
pub struct EcrRepository_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcrRepository_Timeouts {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcrRepository_Timeouts {
    type O = terrars::BlockAssignable<EcrRepository_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRepository_Timeouts {}

impl BuildEcrRepository_Timeouts {
    pub fn build(self) -> EcrRepository_Timeouts {
        EcrRepository_Timeouts { delete: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EcrRepository_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepository_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcrRepository_TimeoutsRef {
        EcrRepository_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRepository_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
