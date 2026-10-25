#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrRepositoryCreationTemplateData_ {
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
    applied_for: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tag_mutability: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<EcrRepositoryCreationTemplate_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tag_mutability_exclusion_filter: std
    ::option
    ::Option<std::vec::Vec<EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter>>,
    dynamic: EcrRepositoryCreationTemplateDynamic,
}

struct EcrRepositoryCreationTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcrRepositoryCreationTemplateData_>,
}

#[derive(Clone)]
pub struct EcrRepositoryCreationTemplate(std::rc::Rc<EcrRepositoryCreationTemplate_>);

impl EcrRepositoryCreationTemplate {
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

    #[doc = "Set the field `custom_role_arn`.\n"]
    pub fn set_custom_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_role_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `image_tag_mutability`.\n"]
    pub fn set_image_tag_mutability(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_tag_mutability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_policy`.\n"]
    pub fn set_lifecycle_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lifecycle_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository_policy`.\n"]
    pub fn set_repository_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().repository_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrRepositoryCreationTemplate_EncryptionConfiguration>>,
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

    #[doc = "Set the field `image_tag_mutability_exclusion_filter`.\n"]
    pub fn set_image_tag_mutability_exclusion_filter(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter>>,
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

    #[doc = "Get a reference to the value of field `applied_for` after provisioning.\n"]
    pub fn applied_for(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.applied_for", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_role_arn` after provisioning.\n"]
    pub fn custom_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `repository_policy` after provisioning.\n"]
    pub fn repository_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<EcrRepositoryCreationTemplate_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> terrars::ListRef<EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()),
        )
    }
}

impl terrars::Referable for EcrRepositoryCreationTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcrRepositoryCreationTemplate { }

impl terrars::ToListMappable for EcrRepositoryCreationTemplate {
    type O = terrars::ListRef<EcrRepositoryCreationTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcrRepositoryCreationTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecr_repository_creation_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrRepositoryCreationTemplate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub applied_for: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildEcrRepositoryCreationTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> EcrRepositoryCreationTemplate {
        let out = EcrRepositoryCreationTemplate(std::rc::Rc::new(EcrRepositoryCreationTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcrRepositoryCreationTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                applied_for: self.applied_for,
                custom_role_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                image_tag_mutability: core::default::Default::default(),
                lifecycle_policy: core::default::Default::default(),
                prefix: self.prefix,
                region: core::default::Default::default(),
                repository_policy: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                image_tag_mutability_exclusion_filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrRepositoryCreationTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepositoryCreationTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcrRepositoryCreationTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `applied_for` after provisioning.\n"]
    pub fn applied_for(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.applied_for", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_role_arn` after provisioning.\n"]
    pub fn custom_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability` after provisioning.\n"]
    pub fn image_tag_mutability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tag_mutability", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `repository_policy` after provisioning.\n"]
    pub fn repository_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<EcrRepositoryCreationTemplate_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tag_mutability_exclusion_filter` after provisioning.\n"]
    pub fn image_tag_mutability_exclusion_filter(
        &self,
    ) -> terrars::ListRef<EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.image_tag_mutability_exclusion_filter", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct EcrRepositoryCreationTemplateDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcrRepositoryCreationTemplate_EncryptionConfiguration>>,
    image_tag_mutability_exclusion_filter: std
    ::option
    ::Option<terrars::DynamicBlock<EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter>>,
}

#[derive(Serialize)]
pub struct EcrRepositoryCreationTemplate_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcrRepositoryCreationTemplate_EncryptionConfiguration {
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

impl terrars::ToListMappable for EcrRepositoryCreationTemplate_EncryptionConfiguration {
    type O = terrars::BlockAssignable<EcrRepositoryCreationTemplate_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRepositoryCreationTemplate_EncryptionConfiguration {}

impl BuildEcrRepositoryCreationTemplate_EncryptionConfiguration {
    pub fn build(self) -> EcrRepositoryCreationTemplate_EncryptionConfiguration {
        EcrRepositoryCreationTemplate_EncryptionConfiguration {
            encryption_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcrRepositoryCreationTemplate_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepositoryCreationTemplate_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrRepositoryCreationTemplate_EncryptionConfigurationRef {
        EcrRepositoryCreationTemplate_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRepositoryCreationTemplate_EncryptionConfigurationRef {
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
pub struct EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
    filter: terrars::PrimField<std::string::String>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter { }

impl terrars::ToListMappable for EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
    type O = terrars::BlockAssignable<EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildEcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
    pub fn build(self) -> EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
        EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

#[allow(dead_code)]
pub struct EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef {
        EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRepositoryCreationTemplate_ImageTagMutabilityExclusionFilterRef {
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
